VAR encountered_pontersons_essence = false
VAR knows_its_a_lab = false

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

== play_loop_phase_1 ==
+ play #play
* [return to woods] -> return_to_woods
* [front desk] -> front_desk
* [loading_bay] -> loading_bay_entrance
+ [End Game]
- 
-> END

== start ==
What is this place? #cass
* [I'm not sure]
    Maybe we should head back?
    ** [Nah - I'm sure it's fine.]
       Ok... if you say s-- <>
    ** [Yeah... it's creepy here...]
        Let's go, before anything-- <>
* [Look's like an abandoned cabin...]
    Was this here before? Why does it look so old?
    ** [It's definitely new - let's check it out!]
        What - wait! <>
    ** [Maybe they left something inside... Wanna check?]
        No -- <>
- What was that? Can you hear... a... Saxophone?  #start_audio
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
        Well, we just got to figure this out. Let's go inside!
    ** [Do you know where you came from?]
        ~ knows_its_a_lab = true
        Of course - I was just in the lab. Come on in!
* [We're in the woods!]
    I can see we're in some woods, but I don't think there should have been woods outside the lab...
    ~ knows_its_a_lab = true
    ** [the lab?]
        Yes - right there, behind me. It's abit smaller and more wooden than it used to be, but the inside did look right! Come on in!
    ** [Well - I don't remember seeing this cabin before, so we might both be lost...]
        That's the lab - it's abit strange that it looks so small and broken on the outside... I think it normally looked different. But inside it's still normal! Let's go!
- &nbsp; #deactivate:pontersons_essence
-> play_loop_intro

== entered_lab ==
{knows_its_a_lab: I guess this is the lab's lobby? | What is this place? It really doesn't match the outside...} #cass
Now where is that spirit... Try following the music and maybe we can find it? #cass
-> play_loop_phase_1

== front_desk ==
Where is everyone? The front desk is empty...
-> play_loop_phase_1

== loading_bay_entrance ==
looks like some kind of loading bay... #deactivate:loading_bay_entrance1 #deactivate:loading_bay_entrance2
-> play_loop_phase_1

== return_to_woods ==
We shouldn't leave yet - we need to figure out what's going on! #cass
-> play_loop_phase_1
