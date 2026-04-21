## [Can't beat the boss? Load a previous save!](https://savepoint.namtao.com)

<img src="logo.jpg" alt="Savepoint logo" height="150px" align="right">

_A command watcher that commits when you fix errors_

Feel free to poke around the [code](https://github.com/NamtaoProductions/savepoint/tree/main/), I would love [feature suggestions](https://github.com/NamtaoProductions/savepoint/issues/new/choose)!

# DEMO

```shell
🏁 SAVEPOINT: Running command...
   Compiling savepoint v0.1.4 (/home/oatman/projects/savepoint)
error: expected one of `!` or `::`, found `SavePoint`
  --> src/main.rs:37:10
   |
37 | uhoh! impl SavePoint<Passing> {
   |            ^^^^^^^^^ expected one of `!` or `::`

error: could not compile `savepoint` (bin "savepoint") due to 1 previous error
🏁 SAVEPOINT: Error!
🏁 SAVEPOINT: Monitoring..
```

```shell
🏁 SAVEPOINT: Running command...
   Compiling savepoint v0.1.4 (/home/oatman/projects/savepoint)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
🏁 SAVEPOINT: Autosaving!
[main 3d65083] SAVEPOINT REACHED!
 2 files changed, 16 insertions(+), 1 deletion(-)
🏁 SAVEPOINT: Monitoring..
```

### State diagram

```mermaid
flowchart LR
PASSING-->|fail|FAILING
FAILING-->|pass; git commit|PASSING
```

> Other transitions are no-ops (such as tests passing while in passing state)

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://brainmade.org/white-logo.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://brainmade.org/black-logo.svg">
  <img align="right" alt="Brain mark." src="https://brainmade.org/black-logo.svg">
</picture>
