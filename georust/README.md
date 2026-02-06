# `georust`

Provides types and methods for geometric concepts (point, line, triangle).

## 2D world

- `Point`
    - Is a `Linvec2` from `linrust`.
- `Line`
    - Is 2 `Point`.
    - Can get `start` which is a `Point`.
    - Can get `end` which is a `Point`.
- `Triangle`
    - Is 3 `Point`.
- `Square`
    - Is 2 `Point` (`top_left` and `bottom_right`).
