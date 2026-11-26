.class public Lcom/example/Greeter;
.super Ljava/lang/Object;
.source "Greeter.java"

.field private static final MAX:I = 0x8

.field private name:Ljava/lang/String;

.method public constructor <init>(Ljava/lang/String;)V
    .registers 2
    invoke-direct {p0}, Ljava/lang/Object;-><init>()V
    iput-object p1, p0, Lcom/example/Greeter;->name:Ljava/lang/String;
    return-void
.end method

.method public greet()Ljava/lang/String;
    .registers 3
    iget-object v0, p0, Lcom/example/Greeter;->name:Ljava/lang/String;
    invoke-static {v0}, Lcom/example/Greeter;->shout(Ljava/lang/String;)Ljava/lang/String;
    move-result-object v1
    return-object v1
.end method

.method private static shout(Ljava/lang/String;)Ljava/lang/String;
    .registers 1
    return-object p0
.end method
