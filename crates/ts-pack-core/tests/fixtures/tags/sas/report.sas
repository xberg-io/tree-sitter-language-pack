libname sales '/data/sales';
%let cutoff = 50;

%macro summarize(ds, var);
  proc means data=&ds;
    var &var;
  run;
%mend summarize;

data work.scores;
  set sales.raw;
  score = points * 2;
run;

data passed;
  update work.scores sales.fixes;
run;

data merged;
  merge work.scores sales.bonus;
  if score > &cutoff;
run;

proc sql;
  create table work.top as
    select * from work.scores;
quit;

%macro footer;
  data _null_;
    total = %summarize(merged, score);
  run;
%mend footer;

proc sort data=merged;
  by score;
run;

%summarize(merged, score);
