# Particle simulator

A simulator where particles spread out evenly over time, if left alone. They achieve this by pushing off from each other. Additionally, some gizmos can be spawned to try to create interesting behaviour.

## Why

I originally started this to help me come up with a ruleset that would create the behaviour I want for a concept I would like to implement in games. That concept is that particles represent mana, energy used by characters (and perhaps other things) to perform actions. As they use the mana, it depletes from the area around them, and density goes down, making further actions weaker or impossible. But over time, new mana flows in to restore the area.

With Bevy, I realized I actually know how to do this now.

I am moderately happy with the behaviour created by the ruleset in this simulator, but I do mean to experiment more. Mainly, I think density doesn't end up actually varying enough for any amount of time, but just adjusting scales might fix that.

## Controls

Left click places a single (positive) particle. `Shift` + a gizmo button deletes the nearest of that type of gizmo within a small radius of the cirsor. `Alt` + a gizmo button deletes all gizmos of that type. `I` toggles inertia mode.

## Gizmos

All gizmos should have simple rules, and either be relevant to my original mana concept, or create fun, emergent behaviour.

- **Emitter** (positive: `=`, negative: `-`): rapidly spawns particles of their own polarity.
- **Deleter** (`!`): instantly deletes any particle in its radius.
- **Attractor** (`@`): attracts particles.
- **Eater** (positive: `]`, negative: `[`]): chases particles of the opposing polarity, deleting ones that are close. After deleting a target number, it spawns as many particles of its own polarity and goes dormant for a short period.

## Implementation details

Every frame, every particle calculates the distance to every other particle, and applies an amount of 'movement' that decreases exponentially with distance for each one. Particles of the same polarity apply this movement away from each other; particles of opposing polarities apply it towards each other. Any relevant gizmos also apply their forces here. Then, the sum of all this movement is applied to the translation.

Comparing all particle locations is very expensive, but I am open to optimizations that involve ignoring any interactions beyond some distance. On my machine, this currently runs at 60 FPS until 1200 particles.

Opposing particles delete each other when they get close.

All distances and positions are calculated wrapping around the edges of the screen.

## Inertia mode

With inertia mode on, movement is not cleared after being applied on each frame. This makes some behaviour more fun, but most behaviour just gets worse, and this is definitely not the main mode. I still mean to try something that's a little in between inertia and non-inertia mode; probably much closer to non-inertia than to inertia.