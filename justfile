# NB: requires Just >= 1.19 and just --unstable

mod clippy '.justfiles/clippy.just'
mod deny '.justfiles/deny.just'
mod fmt '.justfiles/fmt.just'
mod msrv '.justfiles/msrv.just'
mod test '.justfiles/test.just'

before-push:
    # do
    just --unstable fmt
    # run checks
    just --unstable fmt check
    just --unstable clippy
    just --unstable test
    just --unstable msrv
    just --unstable deny
