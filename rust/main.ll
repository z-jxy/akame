; ModuleID = 'main'
source_filename = "main"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

@str_helloWorld = global [11 x i8] c"helloWorld\00"

declare i8 @printf(i8*, ...)

define i32 @hello(i32 %0) {
entry:
  %addtmp = add i32 5, %0
  ret i32 %addtmp
}

define i32 @main(i32 %0, i32 %1) {
entry:
  %calltmp = call i32 @hello(i32 5)
  %calltmp1 = call i8 (i8*, ...) @printf(i8* getelementptr inbounds ([11 x i8], [11 x i8]* @str_helloWorld, i32 0, i32 0))
  ret i32 0
}
