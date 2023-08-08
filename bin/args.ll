; ModuleID = 'main'
source_filename = "main"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"

declare i32 @printf(i8*, ...)

@format_string = constant [4 x i8] c"%s\0A\00"

define i32 @hello(i32 %0) {
entry:
  %addtmp = add i32 5, %0
  ret i32 %addtmp
}

define i32 @main(i32 %argc, i8** %argv) {
entry:
  %calltmp = call i32 @hello(i32 5)

  ; Get the second argument (the string "hey!") from the %argv array
  %arg1 = getelementptr inbounds i8*, i8** %argv, i32 1
  %arg1_str = load i8*, i8** %arg1


  
  ; Print the string "hey!"
  %callprintf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @format_string, i32 0, i32 0), i8* %arg1_str)


  ret i32 0 ; returning 0 for success
}