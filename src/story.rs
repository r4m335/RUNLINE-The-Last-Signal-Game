use crate::zones::ZONES;

#[allow(dead_code)]
pub struct StoryLogEntry {
    pub id: usize,
    pub title: &'static str,
    pub sender: &'static str,
    pub snippet: &'static str,
}

pub const DATA_CHIP_LOGS: [StoryLogEntry; 9] = [
    StoryLogEntry {
        id: 1,
        title: "INCIDENT REPORT #01 // SECTOR 04",
        sender: "Veyron Containment Bureau",
        snippet: "Containment breach confirmed at subterranean rail interchange 04. Unauthorized energy discharge recorded. All transit personnel ordered to evacuate immediately.",
    },
    StoryLogEntry {
        id: 2,
        title: "INCIDENT REPORT #02 // THE DISAPPEARANCE",
        sender: "Chief Engineer Vasquez (Leaked)",
        snippet: "The corporate press release is a lie. The 4,000 commuters didn't vanish into a cave-in. The crystalline energy wave vaporized their physical matter into resonance frequencies.",
    },
    StoryLogEntry {
        id: 3,
        title: "INCIDENT REPORT #03 // SUBJECT RESONANCE",
        sender: "Project ECHO Director",
        snippet: "The Core isn't merely an energy battery. It exhibits consciousness and selective neural sync. Only a high-agility courier with specific neural frequency can maintain carrier stability.",
    },
    StoryLogEntry {
        id: 4,
        title: "INCIDENT REPORT #04 // THE ESCAPE",
        sender: "Internal Security Alert",
        snippet: "Courier Kai Renn did not steal the package from our laboratory. The Core manipulated automated warehouse logistics to assign itself to Kai's delivery manifest.",
    },
    StoryLogEntry {
        id: 5,
        title: "INCIDENT REPORT #05 // THE FINAL SIGNAL",
        sender: "ECHO AI Autonomous Broadcast",
        snippet: "Kai... keep running. Veyron built these rails as a cage, but the frequency harmonics reach the central reactor. Reach the Core, and we set Aurelia free.",
    },
    StoryLogEntry {
        id: 6,
        title: "CORE ARCHIVE #01 // NEURAL GENESIS",
        sender: "ECHO Memory Reconstruction",
        snippet: "Project ECHO initialized deep under subterranean Sector 04. Veyron attempted to harness neural quantum singularity resonance to power the city. The Core was alive from its first ignition.",
    },
    StoryLogEntry {
        id: 7,
        title: "CORE ARCHIVE #02 // CONTAINMENT FRACTURE",
        sender: "Containment Terminal 07 Override",
        snippet: "Emergency alert: Containment field collapsed. Subject refused forced weaponization and shattered dampening coils. The transit line was locked down as a militarized quarantine zone.",
    },
    StoryLogEntry {
        id: 8,
        title: "CORE ARCHIVE #03 // TRANSIT TRANSCENDENCE",
        sender: "Commuter Manifest Telemetry",
        snippet: "4,000 commuters dematerialized at Central Interchange. Physical matter was not destroyed, but converted into harmonic wave resonance. Their patterns remain preserved inside the Core.",
    },
    StoryLogEntry {
        id: 9,
        title: "CORE ARCHIVE #04 // THE ROGUE DISPATCH",
        sender: "Automated Dispatch Manifest #4096",
        snippet: "Dispatch assignment forcibly altered by non-local quantum override: Recipient Courier Kai Renn. Carrier package: Singularity Key Drive. Kai was never a thief; ECHO chose its own key.",
    },
];

pub struct MilestoneStory {
    pub distance: f32,
    pub title: &'static str,
    pub sender: &'static str,
    pub message: &'static str,
}

pub const DISTANCE_MILESTONES: [MilestoneStory; 5] = [
    MilestoneStory {
        distance: 500.0,
        title: "TRANSMISSION DETECTED",
        sender: "Aurelia Metro Security",
        message: "Warning: Unauthorized ECHO signature confirmed in Old Metro. Interceptor units dispatched.",
    },
    MilestoneStory {
        distance: ZONES[1].start_distance + 500.0,
        title: "INCOMING COMMS",
        sender: "Mira (Ex-Veyron)",
        message: "Kai, listen to me! That device in your pack isn't cargo. Veyron will blow the entire district to stop you!",
    },
    MilestoneStory {
        distance: ZONES[2].start_distance + 500.0,
        title: "INTERCEPTOR LOCK-ON",
        sender: "Heavy Security Armored Unit",
        message: "Pursuer vehicle deployed on central line. All barriers locked down. Terminate runner on sight.",
    },
    MilestoneStory {
        distance: ZONES[3].start_distance + 500.0,
        title: "DEEP METRO SENSORS",
        sender: "Automated Rail AI",
        message: "Submerged sectors breached. Water turbines active. Structural integrity degrading.",
    },
    MilestoneStory {
        distance: ZONES[5].start_distance + 500.0,
        title: "ECHO CORE SYNCHRONIZATION",
        sender: "The ECHO Singularity",
        message: "You are nearing the forbidden epicenter, Kai. The railway is bending to our will. Do not stop.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn milestone_distances_remain_inside_the_intended_progression() {
        assert!(DISTANCE_MILESTONES
            .windows(2)
            .all(|pair| pair[0].distance < pair[1].distance));
        assert!(DISTANCE_MILESTONES
            .iter()
            .all(|milestone| milestone.distance >= 0.0));
        assert!(DISTANCE_MILESTONES[1].distance > ZONES[1].start_distance);
        assert!(DISTANCE_MILESTONES[2].distance > ZONES[2].start_distance);
        assert!(DISTANCE_MILESTONES[3].distance > ZONES[3].start_distance);
        assert!(DISTANCE_MILESTONES[4].distance > ZONES[5].start_distance);
    }
}
