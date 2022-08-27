VAR knows_about_portal = false

-> start


== play_loop ==
+ play #play
* [tutorial] -> tutorial
* [encounter pontersons essence] -> encounter_pontersons_essence
* [enter lab] -> entering_the_lab
+ [End Game]
- 
-> END

== end_game ==
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
-> play_loop

== tutorial ==
We're close! But I can't see anything here... #cass
* [What should I do?]
    I think it's a... memory...
    Try following the sound. When you think we're right next to it, let me know
    (by pressing SPACE or ENTER) #deactivate:tutorial_trigger
* [I know what to do!]
    Ok - I'm right behind you! #cass #deactivate:tutorial_trigger
-
-> play_loop

== encounter_pontersons_essence ==
Hello? #ponterson #activate:into_lab_portal
Are you here for the demonstration? #ponterson
* What demonstration?
    The portal demonstration, of course. #ponterson
    ~ knows_about_portal = true
    Come on, it's about to start. #ponterson
* Yes, of course!
    Well, come on in then! It should start in a few minutes. #ponterson
* No... Who are you?
    I'm not quite sure... I just - I know there is a demonstration happening soon. I think it's for something I made... #ponterson
    ** What was it?
        I think it's some kind of portal, but I'm not entirely sure. #ponterson
        ~knows_about_portal = true
    ** You can't remember who you are?
        It is a bit strange...
    -- I'm having some weird memory issues right now... All I can really remember is standing outside the lab, and then realizing - #ponterson
        It's about to start! #ponterson
- 
-> play_loop

== entering_the_lab ==
Wow - I wasn't expecting that cabin to look like this... #cass
* {knows_about_portal} She said something about a portal... Maybe the door was it?
    Maybe you're right... but why put a portal in the middle of the woods? #cass
* Isn't it bigger on the inside?
    It's weird in here - it's so creepy. Where is everyone? #cass
    And what is that music? #cass
* I think I can hear something - maybe it's another memory?
    Ok - just be careful. #cass
-
-> play_loop
