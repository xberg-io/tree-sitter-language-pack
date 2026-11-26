# helpers for filtering records
def active: select(.active == true);

def names: map(.name);

def pick($key): .[$key];

def total(f): reduce .[] as $x (0; . + ($x | f));

.items | map(active) | names
