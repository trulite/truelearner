# Academy S3 storage v1

Academy evidence is stored outside TrueLearner physics. The production bucket
is private and exists in AWS account `444123143883`:

```text
s3://truelearner-academy-444123143883-ap-south-1
region: ap-south-1
prefix: academy/v1
```

The bucket has S3 versioning, BucketOwnerEnforced ownership, SSE-S3 encryption,
and all four public-access blocks enabled.

## Content identity

Every durable payload is addressed by the SHA-256 of its exact stored bytes:

```text
academy/v1/<kind>/<hash[0..2]>/<hash[2..4]>/<hash>
```

Supported kinds are bodies, checkpoints, physical inputs, surfaces, episodes,
thumbnails, and manifests. Hash and byte-length metadata are checked on reads.

Payload objects are immutable by convention. S3 versioning is a recovery
guard, not the Academy history model.

## Publishing an experience

An experience is visible only after all of its immutable payload objects exist
and its manifest has been stored:

```text
payloads -> verify hashes -> manifest -> lineage index/head
```

`publish_a1_experience` implements blobs-before-manifest publication for A1-V.
It stores both live checkpoints, the exact admitted spike stream,
organism/shared raster views, and the complete episode record before publishing
one immutable manifest that references them. Publication happens after physics
and cannot change an Academy result. Missing S3 configuration leaves the
in-memory experience fully replayable.

Lineage indexing and conditional head advancement remain later work.

## Local authentication

Use temporary credentials from the authenticated AWS CLI session. Do not add
access keys to repository files:

```sh
aws login
eval "$(aws configure export-credentials --format env)"
export ACADEMY_S3_BUCKET=truelearner-academy-444123143883-ap-south-1
export ACADEMY_S3_REGION=ap-south-1
export ACADEMY_S3_PREFIX=academy/v1
```

The application uses the standard AWS SDK credential chain. The committed
`academy/.env.example` contains identifiers only and no credentials.
