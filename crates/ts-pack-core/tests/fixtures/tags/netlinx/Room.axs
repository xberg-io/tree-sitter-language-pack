PROGRAM_NAME='Room'

DEFINE_DEVICE

dvTP = 10001:1:0
dvProjector = 5001:1:0

DEFINE_CONSTANT

INTEGER MAX_VOLUME = 100

DEFINE_TYPE

STRUCTURE _Source
{
    CHAR name[32]
    INTEGER input
}

DEFINE_VARIABLE

VOLATILE INTEGER nVolume

DEFINE_FUNCTION INTEGER ClampVolume(INTEGER requested)
{
    IF (requested > MAX_VOLUME)
    {
        RETURN MAX_VOLUME
    }
    RETURN requested
}

DEFINE_CALL 'Power On'
{
    SEND_STRING dvProjector, "'PWR ON', $0D"
}

DEFINE_EVENT

BUTTON_EVENT[dvTP, 1]
{
    PUSH:
    {
        nVolume = ClampVolume(nVolume + 5)
        CALL 'Power On'
    }
}

DATA_EVENT[dvProjector]
{
    ONLINE:
    {
        SEND_STRING dvProjector, "'PWR?', $0D"
    }
}
