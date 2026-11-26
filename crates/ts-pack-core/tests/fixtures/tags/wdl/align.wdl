version 1.1

import "tasks/qc.wdl" as qc

struct Sample {
  String name
  File reads
}

task align {
  input {
    Sample sample
    Int threads = 4
  }

  command <<<
    bwa mem -t ~{threads} ref.fa ~{sample.reads} > out.sam
  >>>

  output {
    File sam = "out.sam"
  }
}

workflow pipeline {
  input {
    Array[Sample] samples
  }

  scatter (s in samples) {
    call align { input: sample = s }
    call qc.check as check_sam { input: sam = align.sam }
  }

  output {
    Array[File] aligned = align.sam
  }
}
