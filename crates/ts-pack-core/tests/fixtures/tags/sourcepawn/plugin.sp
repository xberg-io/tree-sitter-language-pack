#define MAX_PLAYERS 64

int g_Count = 0;

public void OnPluginStart()
{
    PrintToServer("loaded %d", Add(1, 2));
}

int Add(int a, int b)
{
    return a + b;
}
