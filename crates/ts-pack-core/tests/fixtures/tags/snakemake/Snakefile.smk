SAMPLES = ["a", "b"]

rule all:
    input:
        expand("results/{sample}.txt", sample=SAMPLES)

rule process:
    input:
        "data/{sample}.csv"
    output:
        "results/{sample}.txt"
    shell:
        "cp {input} {output}"

def helper(x):
    return x
