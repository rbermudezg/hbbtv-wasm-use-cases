//use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct CuePointsMargin {
    positive: i32,
    negative: i32,
}

struct SubtitlesConfig {
    begin_cue_points_margin: CuePointsMargin,
    end_cue_points_margin: CuePointsMargin,
}

struct GlobalConfiguration {
    subtitles: SubtitlesConfig,
    cue_points_margin: CuePointsMargin,
}

const GLOBAL_CONFIGURATION: GlobalConfiguration = GlobalConfiguration {
    subtitles: SubtitlesConfig {
        begin_cue_points_margin: CuePointsMargin {
            positive: 500,
            negative: 0,
        },
        end_cue_points_margin: CuePointsMargin {
            positive: 500,
            negative: 0,
        },
    },
    cue_points_margin: CuePointsMargin {
        positive: 0,
        negative: 100,
    },
};

pub struct Cuepoint {
    pub id: u32,
    pub ms: i32,
    pub timestopass: i32,
    pub negativemargin: Option<i32>,
    pub positivemargin: Option<i32>,
    pub callback: Box<dyn Fn(i32, &Cuepoint)>,
    pub once: bool,
}

impl PartialEq for Cuepoint {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct Cuepoints {
    cuepoints: Vec<Cuepoint>,
}

impl Cuepoints {
    pub fn new() -> Cuepoints {
        Cuepoints {
            cuepoints: Vec::new(),
        }
    }

    pub fn add_cuepoint(&mut self, mut cuepoint: Cuepoint) -> &Cuepoint {
        cuepoint.timestopass = 0;
        self.cuepoints.push(cuepoint);
        self.cuepoints.last().unwrap()
    }

    pub fn get_cuepoints_by_time(&self, ms: i32) -> Vec<&Cuepoint> {
        let mut found = Vec::new();
        for cuepoint in &self.cuepoints {
            let negativemargin = cuepoint
                .negativemargin
                .unwrap_or(GLOBAL_CONFIGURATION.cue_points_margin.negative);
            let positivemargin = cuepoint
                .positivemargin
                .unwrap_or(GLOBAL_CONFIGURATION.cue_points_margin.positive);
            if ms >= cuepoint.ms - negativemargin && ms <= cuepoint.ms + positivemargin {
                found.push(cuepoint);
            }
        }
        found
    }

    pub fn remove_cuepoint(&mut self, cues: Vec<&Cuepoint>) {
        self.cuepoints.retain(|cue| !cues.contains(&cue));
    }

    pub fn check_cuepoints(&mut self, ms: i32) {
        let cues = self.get_cuepoints_by_time(ms);
        let mut to_remove_indices = Vec::new();

        for (index, cue) in cues.iter().enumerate() {
            if cue.timestopass <= 0 {
                if cue.once {
                    to_remove_indices.push(index);
                }
                log(&format!("ms {} {}", ms, cue.id));
                (cue.callback)(ms, cue);
            }
        }

        for index in to_remove_indices.iter().rev() {
            self.cuepoints.remove(*index);
        }
        for cue in self.cuepoints.iter_mut() {
            cue.timestopass -= 1;
        }
    }
}

// Assuming GLOBALCONFIGURATION is defined elsewhere in the Rust code.
// You will need to define the GLOBALCONFIGURATION with CUEPOINTSMARGIN struct or similar global configuration.

// Also, the callback in Cuepoint struct is a trait object that allows for different function signatures.
// You may need to adjust the type based on the actual callback signatures you expect.
