(module
  (global $EXITCODE (mut i32) (i32.const 0))
  (func $_init
    i32.const 1
    global.set $EXITCODE ;; グローバル変数に1をセット
  )
  (func $exitcode (result i32)
    global.get $EXITCODE
  )
  (export "_initialize" (func $_init))
  (export "exitcode" (func $exitcode))
)
