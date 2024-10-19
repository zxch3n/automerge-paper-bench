# Loro benchmark on Automerge paper dataset

Applying
[Automerge Paper dataset](https://github.com/automerge/automerge-perf/tree/master/edit-by-index)
to Loro 100 times.

- 18,231,500 single-character insertion operations
- 7,746,300 single-character deletion operations
- 25,977,800 operations totally
- 10,485,200 characters in the final document

# Performance

All benchmarks results below were performed on a MacBook Pro M1 2020.

- Apply time taken: 13.26619525s
- Snapshot size: 23,433,380 bytes

| name             | task                       | time                       |
| ---------------- | -------------------------- | -------------------------- |
| New Snapshot     | Parse                      | 17.944514ms +- 169.152µs   |
|                  | Parse+ToString             | 19.675752ms +- 47.009µs    |
|                  | Parse+ToString+Edit        | 117.174317ms +- 184.633µs  |
|                  | Parse+ToString+Edit+Export | 249.908687ms +- 1.657624ms |
| Shallow Snapshot | Parse                      | 14.127859ms +- 71.793µs    |
|                  | Parse+ToString             | 16.024285ms +- 183.136µs   |
|                  | Parse+ToString+Edit        | 112.558132ms +- 404.347µs  |
|                  | Parse+ToString+Edit+Export | 204.917482ms +- 402.789µs  |
