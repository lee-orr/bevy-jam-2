VAR encountered_pontersons_essence = false

-> start


== end_game ==
+ [End Game]
- 
-> END

== play_loop_intro ==
+ play #play
* [tutorial] -> tutorial
* [encounter_pontersons_essence] -> encounter_pontersons_essence
* {encountered_pontersons_essence} [enter lab] -> entered_lab
+ [End Game]
- 
-> END

== start ==
What is this place? #cass
* [I'm not sure]
    Maybe we should head back? #cass
    ** [Nah - I'm sure it's fine.]
       Ok... if you say s-- <>
    ** [Yeah... it's creepy here...]
        Let's go, before anything-- <>
* [Look's like a treehouse]
    Was this here before? Why does it look so old? #cass
    ** [It's definitely new - let's check it out!]
        What - wait! <>
    ** [Maybe they left something inside... Wanna check?]
        No -- <>
- What was that? Can you hear... a... clarinet? #cass  #start_audio
-> play_loop_intro

== tutorial ==
We're close! But I can't see anything here... #cass
* [What should I do?]
    I think it's a... spirit...
    Try following the sound. When you think we're right next to it, let me know
    (by pressing SPACE or ENTER) #deactivate:tutorial_trigger
* [I know what to do!]
    Ok - I'm right behind you! #cass #deactivate:tutorial_trigger
-
-> play_loop_intro

== encounter_pontersons_essence ==
~ encountered_pontersons_essence = true
Hello there! Do you know where we are? #ponterson #activate:into_lab_portal
* [What are you?]
    Hmmm. Interesting question - I don't really know.
    My memory seems to be having some issues - but that just means there's something exciting to figure out!
    ** [You really have no idea?]
        None at all! But I'm sure we can figure this out together. Come on in!
    ** [You seem weirdly chill about this...]
        Well, we just got to figure this out. Come on - let's go to the lab!
    ** [Do you know where you came from?]
        Of course - I was just in the lab. Come on in!
* [We're in the woods!]
    I can see we're in some woods, but I don't think there should have been woods outside the lab...
    ** [the lab?]
        Yes - right there, behind me. It's abit smaller and more wooden than it used to be, but the inside did look right! Come on in!
    ** [Well - I don't remember seeing this cabin before, so we might both be lost...]
        That's the lab - it's abit strange that it looks so small and broken on the outside... I think it normally looked different. But inside it's still normal! Let's go!
-
-> play_loop_intro

== entered_lab ==
Entered the lab
-> end_game
