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
- A small discussion w/ Dr. Ponterson's "essence fragment"/memory, trying
to figure out where it is, and then rushing into the lab with "The demonstration should happen soon!" #activate:into_lab_portal
-> play_loop

== play_loop ==
+ [play #play]
+ [End Game]
- 
-> END

== end_game ==
+ [End Game]
- 
-> END