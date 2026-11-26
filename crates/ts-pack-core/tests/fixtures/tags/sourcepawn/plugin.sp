#include <sourcemod>

#define MAX_PLAYERS 64
#define SQUARE(%1) ((%1) * (%1))

enum Team
{
    Team_None,
    Team_Red,
    Team_Blue
}

enum struct PlayerStats
{
    int kills;
    int deaths;

    float Ratio()
    {
        return float(this.kills) / float(this.deaths + 1);
    }
}

typedef ScoreCallback = function void (int client, int score);

typeset EventCallback
{
    function void (int client);
    function void (int client, int team);
}

methodmap Counter < Handle
{
    public Counter(int start)
    {
        return view_as<Counter>(CreateArray(1, start));
    }

    public void Increment()
    {
        PrintToServer("increment");
    }

    public ~Counter()
    {
        PrintToServer("closed");
    }
}

native int GetScore(int client);

forward void OnScoreChanged(int client, int score);

int g_Count = 0;

public void OnPluginStart()
{
    Counter counter = new Counter(0);
    counter.Increment();
    PrintToServer("loaded %d", Add(SQUARE(2), MAX_PLAYERS));
}

int Add(int a, int b)
{
    int sum = a + b;
    return sum;
}
