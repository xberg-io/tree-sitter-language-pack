package Vehicles {
    private import ScalarValues::*;

    attribute def Speed :> Real;
    enum def Color {
        enum red;
        enum blue;
    }
    item def Fuel;
    port def FuelPort {
        out item fuel : Fuel;
    }
    interface def FuelInterface {
        end supplier : FuelPort;
        end consumer : FuelPort;
    }
    connection def Mounting {
        end part chassis;
        end part motor;
        end chassisEnd part frame : Vehicle;
    }
    allocation def Deployment;
    flow def FuelFlow;
    metadata def Safety;
    occurrence def Trip;
    individual def MyCar :> Vehicle;
    state def EngineStates;
    action def Drive {
        in speed : Speed;
    }
    calc def Range {
        in fuel : Real;
    }
    constraint def MassLimit {
        in mass : Real;
    }
    requirement def MassRequirement {
        subject vehicle : Vehicle;
    }
    concern def Cost;
    use case def Commute {
        actor driver : Person;
        stakeholder owner : Person;
        objective goal;
    }
    analysis def FuelAnalysis;
    verification def MassTest;
    case def Scenario;
    viewpoint def SafetyViewpoint;
    view def SafetyView;
    rendering def TableRendering;
    #Safety def Brake;

    part def Engine;
    part def Person;

    part def Vehicle {
        attribute mass : Real;
        part engine : Engine;
        part wheels : Wheel[4] subsets components;
        port fuelIn : FuelPort;
        ref driver : Person;
        item cargo : Fuel;
        action drive : Drive;
        state states : EngineStates;
        constraint massLimit : MassLimit;
        requirement massReq : MassRequirement;
        connection mount : Mounting;
        interface fuelLink : FuelInterface;
        allocation deploy : Deployment;
        flow fuelFlow : FuelFlow;
        calc range : Range;
        analysis fuelAnalysis : FuelAnalysis;
        verification massTest : MassTest;
        use case commute : Commute;
        case scenario : Scenario;
        view safetyView : SafetyView;
        rendering table : TableRendering;
        occurrence trip : Trip;
        individual myCar : MyCar;
        concern cost : Cost;
        viewpoint safety : SafetyViewpoint;
        enum color : Color;
        event occurrence started;
        event arrival : Trip;
        timeslice morning;
        snapshot parked;
        variation trimLevel : Engine {
            variant part basic;
        }
        exhibit state running : EngineStates;
        metadata safetyInfo : Safety;
        #Safety part brakes;
        #Safety brakeLight;
        speedLimit : Speed;
    }
}
