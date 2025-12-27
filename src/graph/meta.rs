//! Meta is the information that "flows backward" in the graph that the rest of normal data
//!

/// A quality norm used to find a tradeoff between quality and performance
///
/// This is an "informative demand" to nodes, wich then can decide during
/// execution to comply with the demand, by  defining its own level of quality, ignore completely, or
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Quality {
    /// The highest reasonable quality setting, used for rendering
    Highest,

    /// A balanced quality, made to be used the most, should be able to run in real time
    Balanced,

    /// A performance setting, must be able to run in real time on most computers
    Performance,

    /// The lowest quality, used to be the most
    /// Used to make a placeholder, e.g. when a video scene switcher execute one
    /// of its input, but does not render it to the screen, it should probably
    /// be executed anyway to have a temporal consistency (feedback loops)
    Lowest,
}

/// Meta is the information given to each node to inform the context of its
/// execution, it "flows backward"
#[derive(Debug, Clone, Copy)]
pub struct Meta {
    /// A tick incrementing each frame of execution
    pub tick: u64,

    /// A quality norm used to find a tradeoff between quality and performance
    pub quality: Quality,
}

// impl Meta {
//     fn tick(&self) -> u64 {
//         self.tick
//     }

//     fn quality(&self) -> Quality {
//         self.quality
//     }
// }
