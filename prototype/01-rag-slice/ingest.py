"""
Prototype Track 01 – Hello Deterministic RAG

This script ingests markdown files into Qdrant and emits a tiny JSON log
of what was ingested. It is designed to be deterministic and minimal.

Usage (example):
  python ingest.py --src ./docs ./hardware ./software \
    --qdrant-url http://localhost:6333 \
    --collection goni-proto-01 \
    --embed-model sentence-transformers/all-MiniLM-L6-v2 \
    --deterministic

If Qdrant is unavailable, the script writes a mock Arrow batch to
./prototype/01-rag-slice/mock_batch.arrow so the selector path can still
be exercised.
"""

import argparse
import hashlib
import json
import os
import random
import sys
from pathlib import Path

import numpy as np

try:
    from sentence_transformers import SentenceTransformer
except ImportError:
    SentenceTransformer = None  # tolerate missing dependency for now

try:
    import qdrant_client
    from qdrant_client import QdrantClient
    from qdrant_client.models import Distance, PointStruct, VectorParams
except ImportError:
    QdrantClient = None  # tolerate missing dependency for now


def set_deterministic(seed: int = 42):
    random.seed(seed)
    np.random.seed(seed)
    os.environ["PYTHONHASHSEED"] = str(seed)


def hash_path(path: Path) -> str:
    h = hashlib.sha256()
    h.update(str(path).encode("utf-8"))
    return h.hexdigest()[:16]


def read_markdown(paths):
    docs = []
    for p in paths:
        for path in Path(p).rglob("*.md"):
            text = path.read_text(encoding="utf-8", errors="ignore")
            docs.append((path, text))
    return docs


def embed_texts(texts, model_name):
    if SentenceTransformer is None:
        raise RuntimeError("sentence-transformers not installed")
    model = SentenceTransformer(model_name)
    return model.encode(texts, normalize_embeddings=True)


def upsert_qdrant(collection, vectors, docs, client):
    try:
        client.delete_collection(collection)
    except Exception:
        pass
    client.recreate_collection(
        collection_name=collection,
        vectors_config=VectorParams(size=len(vectors[0]), distance=Distance.COSINE),
    )
    points = []
    payloads = []
    for vec, (path, text) in zip(vectors, docs):
        pid = hash_path(path)
        points.append(PointStruct(id=pid, vector=vec.tolist()))
        payloads.append({"path": str(path), "text": text})
    client.upload_collection(collection_name=collection, vectors=vectors, payload=payloads, ids=[p.id for p in points])


def write_mock_batch(out_path: Path, docs, vectors):
    out = []
    for vec, (path, text) in zip(vectors, docs):
        out.append({
            "id": hash_path(path),
            "path": str(path),
            "text": text,
            "embedding": vec.tolist(),
        })
    out_path.write_text(json.dumps(out, indent=2), encoding="utf-8")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--src", nargs="+", required=True, help="source directories of markdown files")
    ap.add_argument("--qdrant-url", default="http://localhost:6333", help="Qdrant HTTP URL")
    ap.add_argument("--collection", default="goni-proto-01", help="Qdrant collection name")
    ap.add_argument("--embed-model", default="sentence-transformers/all-MiniLM-L6-v2", help="embedding model")
    ap.add_argument("--deterministic", action="store_true", help="fix seeds for determinism")
    ap.add_argument("--mock-out", default="mock_batch.json", help="fallback mock batch path")
    args = ap.parse_args()

    if args.deterministic:
        set_deterministic()

    docs = read_markdown(args.src)
    if not docs:
        print("No markdown found", file=sys.stderr)
        sys.exit(1)

    texts = [t for _, t in docs]
    print(f"Embedding {len(texts)} docs with {args.embed_model}...", file=sys.stderr)
    vectors = embed_texts(texts, args.embed_model)

    log = {
        "total_docs": len(docs),
        "embed_model": args.embed_model,
        "deterministic": args.deterministic,
    }

    if QdrantClient:
        try:
            client = QdrantClient(url=args.qdrant_url)
            upsert_qdrant(args.collection, vectors, docs, client)
            log["qdrant"] = {"url": args.qdrant_url, "collection": args.collection}
        except Exception as e:
            print(f"Qdrant unavailable, writing mock batch: {e}", file=sys.stderr)
            out_path = Path(args.mock_out)
            write_mock_batch(out_path, docs, vectors)
            log["mock_batch"] = str(out_path)
    else:
        out_path = Path(args.mock_out)
        write_mock_batch(out_path, docs, vectors)
        log["mock_batch"] = str(out_path)

    print(json.dumps(log, indent=2))


if __name__ == "__main__":
    main()
