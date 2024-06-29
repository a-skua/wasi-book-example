(module
  (import "reactor" "exitcode" (func $exitcode (result i32)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))
  (memory $memory 1)
  (func $_start
    call $exitcode
    call $proc_exit
  )
  (export "_start" (func $_start))
  (export "memory" (memory $memory))
)
