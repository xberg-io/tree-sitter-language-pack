%macro report(ds);
  proc print data=&ds;
  run;
%mend report;

data work.scores;
  score = 1;
run;

%report(work.scores);
