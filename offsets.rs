// // https://quahacks.com/
// 2024-06-28 23:53:41.889494600 UTC

#![allow(non_upper_case_globals, unused)]

pub mod cs2_dumper {
    pub mod offsets {
        // Module: client.dll
        pub mod client_dll {
            pub const dwCSGOInput: usize = 0x1A27F10;
            pub const dwEntityList: usize = 0x19BDE58;
            pub const dwGameEntitySystem: usize = 0x1ADCCA8;
            pub const dwGameEntitySystem_highestEntityIndex: usize = 0x1510;
            pub const dwGameRules: usize = 0x1A1B748;
            pub const dwGlobalVars: usize = 0x1817638;
            pub const dwGlowManager: usize = 0x1A1AE30;
            pub const dwLocalPlayerController: usize = 0x1A0DA88;
            pub const dwLocalPlayerPawn: usize = 0x1823A08;
            pub const dwPlantedC4: usize = 0x1A25288;
            pub const dwPrediction: usize = 0x18238C0;
            pub const dwSensitivity: usize = 0x1A1C418;
            pub const dwSensitivity_sensitivity: usize = 0x40;
            pub const dwViewAngles: usize = 0x1A2D328;
            pub const dwViewMatrix: usize = 0x1A1FDB0;
            pub const dwViewRender: usize = 0x1A20548;
            pub const dwWeaponC4: usize = 0x19C1A20;
        }
        // Module: engine2.dll
        pub mod engine2_dll {
            pub const dwBuildNumber: usize = 0x52F834;
            pub const dwNetworkGameClient: usize = 0x52EBA0;
            pub const dwNetworkGameClient_clientTickCount: usize = 0x178;
            pub const dwNetworkGameClient_deltaTick: usize = 0x278;
            pub const dwNetworkGameClient_isBackgroundMap: usize = 0x281477;
            pub const dwNetworkGameClient_localPlayer: usize = 0xF0;
            pub const dwNetworkGameClient_maxClients: usize = 0x270;
            pub const dwNetworkGameClient_serverTickCount: usize = 0x174;
            pub const dwNetworkGameClient_signOnState: usize = 0x260;
            pub const dwWindowHeight: usize = 0x5F0424;
            pub const dwWindowWidth: usize = 0x5F0420;
        }
        // Module: inputsystem.dll
        pub mod inputsystem_dll {
            pub const dwInputSystem: usize = 0x387F0;
        }
        // Module: matchmaking.dll
        pub mod matchmaking_dll {
            pub const dwGameTypes: usize = 0x1A41C0;
            pub const dwGameTypes_mapName: usize = 0x120;
        }
        // Module: soundsystem.dll
        pub mod soundsystem_dll {
            pub const dwSoundSystem: usize = 0x334E40;
            pub const dwSoundSystem_engineViewData: usize = 0x7C;
        }
    }
}
