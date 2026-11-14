CREATE TABLE documents (
    id TEXT PRIMARY KEY NOT NULL,
    head_revision TEXT NOT NULL,
    FOREIGN KEY (id, head_revision) REFERENCES revisions(document_id, id)
        DEFERRABLE INITIALLY DEFERRED
) STRICT;
CREATE TABLE revisions (
    document_id TEXT NOT NULL REFERENCES documents(id),
    id TEXT NOT NULL,
    sequence INTEGER NOT NULL CHECK(sequence >= 0),
    parent_id TEXT,
    manifest TEXT NOT NULL CHECK(json_valid(manifest)),
    PRIMARY KEY(document_id, id),
    UNIQUE(document_id, sequence),
    FOREIGN KEY(document_id, parent_id) REFERENCES revisions(document_id, id)
) STRICT;
CREATE TABLE claim_versions (
    document_id TEXT NOT NULL,
    version_id TEXT NOT NULL,
    introduced_revision TEXT NOT NULL,
    ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
    payload TEXT NOT NULL CHECK(json_valid(payload)),
    PRIMARY KEY(document_id, version_id),
    UNIQUE(document_id, ordinal),
    FOREIGN KEY(document_id, introduced_revision) REFERENCES revisions(document_id, id)
) STRICT;
CREATE TABLE receipts (
    key TEXT PRIMARY KEY NOT NULL,
    document_id TEXT NOT NULL,
    revision_id TEXT NOT NULL,
    request_hash BLOB NOT NULL CHECK(length(request_hash) = 32),
    proposal TEXT NOT NULL CHECK(json_valid(proposal)),
    outcome TEXT NOT NULL CHECK(json_valid(outcome)),
    FOREIGN KEY(document_id, revision_id) REFERENCES revisions(document_id, id)
) STRICT;
