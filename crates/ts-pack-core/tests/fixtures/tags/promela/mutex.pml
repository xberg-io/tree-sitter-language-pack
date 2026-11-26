/* two workers sharing a lock */
mtype = { request, release };

typedef Msg {
  byte id;
  mtype kind
};

chan lock = [1] of { mtype };

bool critical = false;

proctype worker(byte id) {
  byte local_count = 0;
  lock ! request;
  critical = true;
  critical = false
}

init {
  run worker(0);
  run worker(1)
}

never safety {
  skip
}

ltl mutual { [] !critical }
