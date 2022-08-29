VAR encountered_pontersons_essence = false
VAR knows_its_a_lab = false
VAR knows_about_the_bet = false
VAR knows_it_was_a_portal = false
VAR knows_it_was_unstable = false
VAR bricksworth_knew_it_was_unstable = false
VAR ponterson_believes_its_dangerous = false
VAR ponterson_made_adjustments_before_the_demonstration = false
VAR rollins_admired_the_view = false
VAR alvernis_and_bricksworth_had_an_affair = false
VAR alvernis_and_bricksworth_broke_up = false
VAR bricksworth_cant_share_credit = false
VAR alverniss_was_in_the_control_room = false
VAR ponterson_started_the_portal = false
VAR bricksworth_kicked_out = false



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
* [rollins_and_alverniss_placing_a_bet] -> rollins_and_alverniss_placing_a_bet
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
    I think it's a... memory...
    Try following the sound. When you think we're right next to it, let me know
    (by pressing SPACE or ENTER) #deactivate:tutorial_trigger
* [I know what to do!]
    Ok - I'm right behind you! #cass #deactivate:tutorial_trigger
-
-> play_loop_intro

== encounter_pontersons_essence ==
~ encountered_pontersons_essence = true
I know everything will be ok... #ponterson #activate:into_lab_portal
* [Hello?]
* [What are you?]
* [What is happening?]
    I think we're seeing what happened here... #cass
- Look - I have to go in. Gotta get back in the lab #ponterson
    the demonstration is starting any second now. 
    ~ knows_its_a_lab = true
  [...]
  I love you too. I'll see you tonight.
- &nbsp; #deactivate:pontersons_essence
-> play_loop_intro

== entered_lab ==
{knows_its_a_lab: I guess this is the lab's lobby? | What is this place? It really doesn't match the outside...} #cass
Now what happened here... Try following the music and maybe we can find it? #cass
-> play_loop_phase_1

== front_desk ==
Where is everyone? The front desk is empty...
-> play_loop_phase_1

== loading_bay_entrance ==
looks like some kind of loading bay... #deactivate:loading_bay_entrance1 #deactivate:loading_bay_entrance2
-> play_loop_phase_1

== rollins_and_alverniss_placing_a_bet ==
So, Alverniss, do you think this will work? #rollins
Well Corpral, it seems far fetched, but Dr. Ponterson knows her stuff... #alverniss
What would you say to a small wager? #rollins
* [Looks like they weren't really confident {knows_it_was_a_portal: in the portal... |in... whatever it is}]
    It definitely looks like something went wrong... they were probably right #cass
* {not knows_it_was_a_portal} [This must be about the demonstration!]
    I think you're right - but what was the demonstration about? #cass
* Who are these? Guests for the demonstration?
    Probably... #cass
- Let's look around abit more - maybe we can figure it out! 
~ knows_about_the_bet = true
-> play_loop_phase_1

== bricksworth_and_ponterson_fighting ==
We need to delay the demonstration! #ponterson
We can't delay anymore - it's happening tonight. End of story. #bricksworth
Mr. Bricksworth - it's too dangerous... The portal is still unstable. #ponterson
End. Of. Story. #bricksworth
* {knows_about_the_bet} [I guess that wager was actually a good idea...]
    Yeah - looks like no one really believed this portal could work. #cass
* [A portal? That's insane!]
    No wonder it didn't go well - an unstable portal... #cass
* [Why would Bricksworth do something so risky?]
    I bet there was something more going on there #cass
- &nbsp; 
~ knows_it_was_a_portal = true
~ knows_it_was_unstable = true
~ bricksworth_knew_it_was_unstable = true
-> play_loop_phase_1

== seeing_portal_first_time ==
Wow! {knows_it_was_a_portal:This must be the portal.|What is this?} #cass #deactivate:seeing_portal_first_time
-> play_loop_phase_1

== enter_control_room ==
Looks like some kind of control room... #cass #deactivate:enter_control_room
-> play_loop_phase_1

== enter_power_room ==
Look at all those power cells. {knows_it_was_a_portal:The portal|Whatever it is} must take a lot of power... #cass #deactivate:enter_power_room
-> play_loop_phase_1

== observation_deck ==
This must be where they watched the demonstration #cass #deactivate:observation_deck
-> play_loop_phase_1

== enter_meeting_room ==
That's a massive table... must be the meeting room #cass #deactivate:enter_meeting_room
-> play_loop_phase_1

== ceo_office ==
Mr. Bricksworth - CEO. This must be his office #cass #deactivate:ceo_office
-> play_loop_phase_1

== too_dangerous ==
I can't believe Bricksworth is letting the military send someone - especially a hawk like Cpl. Rollins - #ponterson
to see the demonstration.
Doesn't he know how dangerous it is if a military gets a hold of it?
* {bricksworth_knew_it_was_unstable} [Looks like there was more to Bricksworth & Ponterson's disagreement...]
    Definitely... Why would she still be here? #cass
* {knows_about_the_bet} [Well, at least Rollins was betting this wouldn't work...]
    But if it did, sounds like he'd jump all over this. #cass
* [Sending troops anywhere in an instant? That sounds terrifying]
    No wonder Ponterson was worried about it. #cass
- &nbsp; 
~ponterson_believes_its_dangerous = true
~ponterson_made_adjustments_before_the_demonstration = true
-> play_loop_phase_1

== rollins_admiring_the_view ==
What are you doing here? #ponterson
Just admiring your work, Dr. #rollins
You shouldn't be in here - this area is full of --- #ponterson
Of course - I apologize - where is the viewing deck? #rollins
* {knows_about_the_bet} [Didn't Rollins bet against it working?]
    Maybe they were here for another reason... #cass
* {knows_it_was_unstable} [Hopefulyl Rollins being here didn't make things even more unstable...]
- &nbsp; 
~rollins_admired_the_view = true
-> play_loop_phase_1

== alverniss_and_bricksworth_bicker ==
Look, I'm just here as an investor. Nothing else. #alverniss
But dear - I was -- #bricksworth
Don't call me that - you don't get to call me that any more. #alverniss
Darling, please forgive #bricksworth
Goodbye, Mr. Bricksworth - I'll see you at the demonstration. #Alverniss
Dammit! #bricksworth
- &nbsp;
~ alvernis_and_bricksworth_had_an_affair = true
~ alvernis_and_bricksworth_broke_up = true
-> play_loop_phase_1

== alverniss_and_bricksworth_dance ==
Oh I love this music #alverniss
Shall we? #bricksworth
[...]
We're so close, Darling - I think we'll have something ready in a few months
Really? #alverniss
I've made a lot of progress - #bricksworth
You mean, Ponterson has? #alverniss
Yes... sorry-#bricksworth
- &nbsp; 
~ alvernis_and_bricksworth_had_an_affair = true
~ bricksworth_cant_share_credit = true
-> play_loop_phase_1

== alverniss_in_the_control_room ==
These panels look... complicated. #alverniss
I wonder how they work?
Nevermind - the demonstration is starting soon - I should get out of here...
- &nbsp; 
~ alverniss_was_in_the_control_room = true
-> play_loop_phase_1

== ponterson_starting_demonstration ==
Turning on the containment field #ponterson
All systems are ready.
[whispered] It'll be ok...
Initiating portal in 3 - 2 - 1 --
- &nbsp; 
~ ponterson_started_the_portal = true
-> play_loop_phase_1

== bricksworth_will_be_fired ==
Mr. Bricksworth, the board has seen fit to - #alverniss
Please don't do this dear #bricksworth
Stop it - the board has decided to give you one last chance. #alverniss
Oh - thank you. #bricksworth
Don't thank me. You have 2 months, and if we don't see a working prototype by then... #alverniss
- &nbsp; 
~ bricksworth_kicked_out = true
-> play_loop_phase_1


== return_to_woods ==
We shouldn't leave yet - we need to figure out what's going on! #cass
-> play_loop_phase_1


== deduction ==
So, what do you think happened here? #cass
+ {knows_it_was_a_portal} [I think the portal had an accident...]
    Why?
        ++ {knows_it_was_unstable}  [We know that Bricksworth pushed Ponterson to get it done before she was ready.]
            That makes sense...
                -> end_game
        ++ [I don't know... maybe we should keep looking?]
+ [I think it was sabotaged]
    What makes you think that?
        ++ {alvernis_and_bricksworth_broke_up} [Alverniss & Bricksworth had an affair, and broke up. Maybe they wanted revenge?]
            But how?
            +++ {alverniss_was_in_the_control_room} Alverniss messed with the panels in the control room.
                I think you're right! It must have been them!
                    -> end_game
            +++ [Maybe they... Nevermind - let's keep looking]
        ++ {rollins_and_alverniss_placing_a_bet} [Rollins bet the portal wouldn't work]
            Yes - but you think he could sabotage it?
                +++ {rollins_admired_the_view} He was caught right next to the portal.
                    I bet you're right. It was the Corpral.
                        -> end_game
                +++ [I mean - he's probably used to having his way. But we should find actual proof.]
        ++ {ponterson_believes_its_dangerous} [Ponterson thought it was too dangerous]
            And she also seemed really nervous all day {knows_it_was_unstable:- and tried to delay|}. But do we have any evidence?
                +++ {ponterson_made_adjustments_before_the_demonstration} [She did make adjustments right before the demonstration]
                    That's true... maybe you're right.
                        -> end_game
                +++ {ponterson_started_the_portal} [She designed it, and started the portal. And she was nervous about it]
                    You're probably right.
                        -> end_game
                +++ [Not yet... Let's see if we can find any]
        ++ {bricksworth_kicked_out} [Bricksworth did it.]
            Why would he do it? He's the CEO!
            +++ [Yes - but he was going to be kicked out if this didn't work]
                I mean sure, but that isn't enough on it's own.
            +++ {alvernis_and_bricksworth_broke_up} [Yes - but he and Averniss broke up. And then she tried to kick him out - and was about to succeed.]
                He definitely sounded like he wasn't willing to accept any of that...
                Maybe he was egotistical enough?
                    -> end_game
        ++ [It's the most exciting option! Let's see if it's right]
+ [I have no idea... let's keep going]
-
 -> play_loop_phase_1