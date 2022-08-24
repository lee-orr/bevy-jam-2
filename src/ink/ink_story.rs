use bevy::prelude::*;
use inkling::{Story, LineBuffer, Prompt, InklingError, read_story_from_string, Variable};

use super::ink_asset::*;

pub struct InkStory {
    story: Story,
}

pub struct StoryEvent {
    pub lines: LineBuffer,
    pub prompt: Prompt
}

impl InkStory {
    pub fn new(
        handle: &Handle<InkAsset>,
        assets: &Assets<InkAsset>,
    ) -> Option<Self> {
        let asset = assets.get(handle);
        match asset {
            Some(asset) => {
                let mut story = read_story_from_string(&asset.story);
                match story {
                    Ok(mut story) => {
                        if let Err(_) = story.start() {
                            return None;
                        }

                        Some(Self { story })
                    }
                    Err(err) => None,
                }
            }
            None => None,
        }
    }

    pub fn resume_story(
        &mut self,
    ) -> Result<StoryEvent, InklingError> {
        let mut buffer: LineBuffer = vec![];
        let prompt =
            self.story.resume(&mut buffer);
        match prompt {
            Ok(prompt) => Ok(StoryEvent { lines: buffer, prompt }),
            Err(e) => Err(e),
        }
    }

    pub fn resume_story_with_event(&mut self, event_writer: &mut EventWriter<StoryEvent>) {
        let resumed = self.resume_story();
        if let Ok(resumed) = resumed {
            event_writer.send(resumed)
        }
    }

    pub fn make_choice(&mut self, choice: usize) -> Result<(), InklingError> {
        self.story.make_choice(choice)
    }

    pub fn move_to(
        &mut self,
        knot: &str,
        stitch: Option<&str>,
    ) -> Result<(), InklingError> {
        self.story.move_to(knot, stitch)
    }

    pub fn get_variable(&self, name: &str) -> Result<Variable, InklingError> {
        self.story.get_variable(name)
    }

    pub fn set_variable<T: Into<Variable>>(&mut self, name: &str, value: T) -> Result<(), InklingError> {
        self.story.set_variable(name, value)
    }
}
