import json
import random
import sys
from pathlib import Path

import numpy as np
import pytest

ROOT = Path(__file__).parent
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))

import ingest  # noqa: E402  (import after sys.path tweak)


def test_hash_path_is_stable_and_unique(tmp_path):
    path_a = tmp_path / "a.md"
    path_b = tmp_path / "b.md"
    path_a.write_text("a")
    path_b.write_text("b")

    hash_a_first = ingest.hash_path(path_a)
    hash_a_second = ingest.hash_path(path_a)
    hash_b = ingest.hash_path(path_b)

    assert hash_a_first == hash_a_second
    assert len(hash_a_first) == 16
    assert hash_a_first != hash_b


def test_read_markdown_filters_md_only(tmp_path):
    md_file = tmp_path / "doc.md"
    txt_file = tmp_path / "note.txt"
    md_file.write_text("# title")
    txt_file.write_text("ignore me")

    docs = ingest.read_markdown([tmp_path])
    paths = {p for p, _ in docs}

    assert paths == {md_file}


def test_set_deterministic_resets_random_streams():
    ingest.set_deterministic(7)
    first = ([random.random() for _ in range(3)], np.random.rand(3).tolist())
    ingest.set_deterministic(7)
    second = ([random.random() for _ in range(3)], np.random.rand(3).tolist())

    assert first == second


def test_embed_texts_calls_sentence_transformer(monkeypatch):
    instances = []

    class FakeModel:
        def __init__(self, name):
            self.name = name
            self.last_normalize = None
            instances.append(self)

        def encode(self, texts, normalize_embeddings):
            self.last_normalize = normalize_embeddings
            return np.array([[len(t), 0.0] for t in texts])

    monkeypatch.setattr(ingest, "SentenceTransformer", FakeModel)

    res = ingest.embed_texts(["hi", "hello"], "fake-model")

    assert instances and instances[0].name == "fake-model"
    assert instances[0].last_normalize is True
    assert res.shape == (2, 2)


def test_embed_texts_raises_when_dependency_missing(monkeypatch):
    monkeypatch.setattr(ingest, "SentenceTransformer", None)
    with pytest.raises(RuntimeError):
        ingest.embed_texts(["text"], "any-model")


def test_main_writes_mock_batch_when_qdrant_missing(monkeypatch, tmp_path, capsys):
    src_dir = tmp_path / "src"
    src_dir.mkdir()
    doc_path = src_dir / "doc.md"
    doc_path.write_text("sample text")
    mock_out = tmp_path / "out.json"

    monkeypatch.setattr(ingest, "QdrantClient", None)
    monkeypatch.setattr(
        ingest,
        "embed_texts",
        lambda texts, model: np.array([[float(i), float(i + 1)] for i in range(len(texts))]),
    )

    argv = [
        "ingest.py",
        "--src",
        str(src_dir),
        "--mock-out",
        str(mock_out),
        "--deterministic",
    ]
    monkeypatch.setattr(sys, "argv", argv)

    ingest.main()
    stdout = json.loads(capsys.readouterr().out)

    assert stdout["total_docs"] == 1
    assert stdout["mock_batch"] == str(mock_out)

    data = json.loads(mock_out.read_text())
    assert len(data) == 1
    record = data[0]
    assert record["path"] == str(doc_path)
    assert record["id"] == ingest.hash_path(doc_path)
    assert record["embedding"] == [0.0, 1.0]
