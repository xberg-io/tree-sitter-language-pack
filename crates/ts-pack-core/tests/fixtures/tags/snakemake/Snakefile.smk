configfile: "config.yaml"

SAMPLES = ["a", "b"]


def helper(wildcards):
    prefix = "data"
    return f"{prefix}/{wildcards.sample}.csv"


class Paths:
    def output(self, sample):
        return f"results/{sample}.txt"


module qc:
    snakefile:
        "rules/qc.smk"


use rule * from qc as qc_*


rule all:
    input:
        expand("results/{sample}.txt", sample=SAMPLES)


checkpoint split:
    output:
        directory("chunks")
    shell:
        "mkdir -p {output}"


rule process:
    input:
        helper
    output:
        "results/{sample}.txt"
    shell:
        "cp {input} {output}"
