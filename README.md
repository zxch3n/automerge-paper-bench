# Loro benchmark on Automerge paper dataset

Applying
[Automerge Paper dataset](https://github.com/automerge/automerge-perf/tree/master/edit-by-index)
to Loro **100 times**.

- 18,231,500 single-character insertion operations
- 7,746,300 single-character deletion operations
- 25,977,800 operations totally
- 10,485,200 characters in the final document

# Performance

All benchmarks results below were performed on a MacBook Pro M1 2020.

- Apply time taken: 13.26619525s

| Snapshot Type    | Size (bytes) |
| ---------------- | ------------ |
| Old snapshot     | 27,347,374   |
| New snapshot     | 23,433,380   |
| Shallow Snapshot | 4,388,215    |

| name             | task                       | time                       |
| ---------------- | -------------------------- | -------------------------- |
| Old Snapshot     | Parse                      | 537.779362ms +- 3.227008ms |
|                  | Parse+ToString             | 568.041745ms +- 1.775865ms |
|                  | Parse+ToString+Edit        | 561.189862ms +- 939.651µs  |
|                  | Parse+ToString+Edit+Export | 1.447087233s +- 22.93437ms |
| New Snapshot     | Parse                      | 17.870013ms +- 48.451µs    |
|                  | Parse+ToString             | 20.19204ms +- 57.2µs       |
|                  | Parse+ToString+Edit        | 118.620566ms +- 180.067µs  |
|                  | Parse+ToString+Edit+Export | 251.031556ms +- 1.600972ms |
| Shallow Snapshot | Parse                      | 14.444623ms +- 113.975µs   |
|                  | Parse+ToString             | 16.84168ms +- 81.361µs     |
|                  | Parse+ToString+Edit        | 113.083084ms +- 184.648µs  |
|                  | Parse+ToString+Edit+Export | 206.066649ms +- 360.419µs  |
