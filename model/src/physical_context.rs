use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum LogicalContext {
    #[default]
    #[serde(rename = "Capacity")]
    Capacity,
    #[serde(rename = "Environment")]
    Environment,
    #[serde(rename = "Network")]
    Network,
    #[serde(rename = "Performance")]
    Performance,
    #[serde(rename = "Security")]
    Security,
    #[serde(rename = "Storage")]
    Storage,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PhysicalContext {
    #[default]
    #[serde(rename = "ACInput")]
    ACInput,
    #[serde(rename = "ACMaintenanceBypassInput")]
    ACMaintenanceBypassInput,
    #[serde(rename = "ACOutput")]
    ACOutput,
    #[serde(rename = "ACStaticBypassInput")]
    ACStaticBypassInput,
    #[serde(rename = "ACUtilityInput")]
    ACUtilityInput,
    #[serde(rename = "ASIC")]
    ASIC,
    #[serde(rename = "Accelerator")]
    Accelerator,
    #[serde(rename = "Back")]
    Back,
    #[serde(rename = "Backplane")]
    Backplane,
    #[serde(rename = "Battery")]
    Battery,
    #[serde(rename = "Board")]
    Board,
    #[serde(rename = "CPU")]
    CPU,
    #[serde(rename = "CPUSubsystem")]
    CPUSubsystem,
    #[serde(rename = "Chassis")]
    Chassis,
    #[serde(rename = "ComputeBay")]
    ComputeBay,
    #[serde(rename = "CoolingSubsystem")]
    CoolingSubsystem,
    #[serde(rename = "DCBus")]
    DCBus,
    #[serde(rename = "Exhaust")]
    Exhaust,
    #[serde(rename = "ExpansionBay")]
    ExpansionBay,
    #[serde(rename = "FPGA")]
    FPGA,
    #[serde(rename = "Fan")]
    Fan,
    #[serde(rename = "Front")]
    Front,
    #[serde(rename = "GPU")]
    GPU,
    #[serde(rename = "GPUSubsystem")]
    GPUSubsystem,
    #[serde(rename = "Intake")]
    Intake,
    #[serde(rename = "LiquidInlet")]
    LiquidInlet,
    #[serde(rename = "LiquidOutlet")]
    LiquidOutlet,
    #[serde(rename = "Lower")]
    Lower,
    #[serde(rename = "Memory")]
    Memory,
    #[serde(rename = "MemorySubsystem")]
    MemorySubsystem,
    #[serde(rename = "Motor")]
    Motor,
    #[serde(rename = "NetworkBay")]
    NetworkBay,
    #[serde(rename = "NetworkingDevice")]
    NetworkingDevice,
    #[serde(rename = "PowerSubsystem")]
    PowerSubsystem,
    #[serde(rename = "PowerSupply")]
    PowerSupply,
    #[serde(rename = "PowerSupplyBay")]
    PowerSupplyBay,
    #[serde(rename = "Pump")]
    Pump,
    #[serde(rename = "Rectifier")]
    Rectifier,
    #[serde(rename = "Room")]
    Room,
    #[serde(rename = "StorageBay")]
    StorageBay,
    #[serde(rename = "StorageDevice")]
    StorageDevice,
    #[serde(rename = "SystemBoard")]
    SystemBoard,
    #[serde(rename = "Transceiver")]
    Transceiver,
    #[serde(rename = "Transformer")]
    Transformer,
    #[serde(rename = "TrustedModule")]
    TrustedModule,
    #[serde(rename = "Upper")]
    Upper,
    #[serde(rename = "VoltageRegulator")]
    VoltageRegulator,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PhysicalSubContext {
    #[default]
    #[serde(rename = "Input")]
    Input,
    #[serde(rename = "Output")]
    Output,
}
