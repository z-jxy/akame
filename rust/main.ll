; ModuleID = 'main'
source_filename = "main"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"

@str_helloWorld = global [11 x i8] c"helloWorld\00"

declare i8 @printf(ptr, ...)

define i32 @hello(i32 %0) {
entry:
  %addtmp = add i32 5, %0
  ret i32 %addtmp
}

define i32 @main(i32 %0, i32 %1) {
entry:
  %calltmp = call i32 @hello(i32 5)
  %calltmp1 = call i8 (ptr, ...) @printf(ptr @str_helloWorld)
  ret i32 0
}
