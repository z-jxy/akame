; ModuleID = 'main'
source_filename = "main"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"

@global_string = global [11 x i8] c"hello world"

declare i32 @printf(ptr, ...)

define i32 @hello(i32 %0) {
entry:
  %addtmp = add i32 5, %0
  ret i32 %addtmp
}

define i32 @main(i32 %0) {
entry:
  %calltmp = call i32 @hello(i32 5)
  %calltmp1 = call i32 (ptr, ...) @printf(ptr @global_string)
  ret i32 %calltmp1
}
