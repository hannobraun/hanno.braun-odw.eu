+++
title   = "My Criteria for CAD Software"
date    = 2021-10-26
updated = 2021-10-28

[extra]
intro = """
I've been doing CAD modeling for a while now, and although I'm far from being an expert, I have developed some opinions about how CAD software should work, and what it should provide. In this note, I lay out my own criteria by which I judge CAD software.
"""

[[extra.changes]]
date = 2021-10-28
text = """
Added "Project Health" as criterion.
"""
+++

Before I start, I'd like to emphasize two things:

- First, as the title says, these are *my* criteria. I'm not telling you what to think or do, I'm just describing what I think would best fit my own use cases and preferences.
- Second, these criteria are neither complete nor final. As I gain more experience and learn more about CAD, I'm sure to think of more to add here, or change my mind about some of them.

But enough prelude. Let's get into it.


### Criteria

#### Overview

Before I go into detail, here's a short overview of the criteria I have come up with so far:

1. The software should be developed by a **healthy project**. Ideally, it is *open source*, *actively developed*, has a *vibrant community*, *documentation* is available, and it is *packaged* for a wide variety of platforms.
1. Some **basic features**, like *Constructive Solid Geometry* and *flexible extrude/sweep operations* should be available.
1. Models should be **defined as code**. The language of that code should be *easy to use*, *powerful*, and *robust*.
1. It should be easy to **view different aspects** of your model.
1. It should be easy to **reference and manipulate existing geometry**.
1. It should support **constraint-based modeling**.
1. 2D **sketches** should be easy to create and manipulate.


#### 1. Project Health

Before getting into a new CAD program, it's worth considering how it is developed, and how viable that development is long-term. While many people make a whole lifestyle out of never upgrading anything, I'm firmly of the opinion that life is change, and to thrive we have to adapt. Doing that is much harder when using stagnant software.

There are a few sub-criteria that I judge the health of a software project by:

- Is it **open source**? There is very good proprietary software too, but open source software can provide a larger amount of control and certainty to users.
- Is it **actively developed**? Without ongoing development, you might run into unfixable limitations sooner or later.
- Does it have a **vibrant community** of users and developers? The more people are involved in a constructive way, the higher the likelihood that the project stays viable in the long term.
- Are **documentation** and other learning materials available? In a pinch, well-written source code might do, if you can read it, but having the selection of a wide variety of materials is much better.
- Is the software **packaged** for a wide variety of platforms? If you have to compile a project from source or, even worse, it doesn't support your system at all, this can cause problems that you're not well-equipped to solve.


#### 2. Basic Features

Some features are extremely useful in CAD software, but at the same time are so pervasive, it makes no sense to break them out as separate criteria. Among those are *Constructive Solid Geometry* and *flexible extrude/sweep operations*. I use both of those in most of my models.


#### 3. Models Defined as Code

I believe that CAD models [should be defined as code](/notes/code-cad-advantages). But that alone isn't enough. The language that this code is written in should fulfil several criteria:

1. It should be **easy to use**. Some general-purpose programming languages are very complex, syntactically heavy, or cumbersome (sometimes for good reasons). I don't think a lot of complexity is needed for this particular use case though. The language should put few hurdles in front of the CAD designer as is practical.
1. At the same time, the language shouldn't be simplistic. Some models still benefit from non-trivial algorithms, and the language should be **powerful** enough to support that.
1. Programming is an error-prone activity. To alleviate that, the language should be **robust**, meaning it should prevent as many errors as possible. 

These three attributes can be in conflict with one another. Lots of Code-CAD programs use dynamically types languages that are easy to use and powerful, but not robust. Languages that are robust are often not easy to use.

Check out my [note about Code-CAD software](/notes/programmatic-cad) for an overview over existing solutions.

#### 4. Different Views

It should be easy to view all different aspects of your model, including intermediate stages of modeling, not just the final result. This seems to be pretty standard in regular GUI-based CAD software, but not in Code-CAD software.

#### 5. Manipulating Existing Geometry

Referencing and manipulating existing geometry can be very useful. Tasks like adding a feature to a specific face can become much easier that way. Other tasks, like adding a chamfer or fillet to an edge, can turn into a real pain without that capability.

#### 6. Constraint-based Modeling

Constraint-based modeling means being able to define geometry by defining the relationship between different parts of it. "Line A must be parallel to line B", for example, or "Lines A and B must be orthogonal and have the same length". Since constraints reference existing geometry, this builds on criterion 4.

In GUI-based CAD software, constraints enable parametric modeling. In Code-CAD software, constraints are less essential, since that approach provides other ways of creating parametric models (and actually, if you keep your code clean, that aspect comes pretty natural).

But still, even in Code-CAD software, constraint-based modeling can be very useful. In some situations, a few well-placed constraints can save a lot of cumbersome code.

#### 7. Easy Sketching

2D sketches are extremely useful (even essential) in CAD modeling. Unfortunately, using Code-CAD (criterion 2) often means you have to create sketches by typing up the code that defines them. And as much as I like typing up code that defines things, I think it is very unnatural in this case, just making things harder than they need to be.

This is even worse, if you modify an existing sketch. Typically, you'll have built more geometry on top of it, and since Code-CAD software often only shows you the final result (criterion 3), you can't even see the sketch you are changing, without shuffling code around.

I think there is room for a hybrid approach here, where models are defined in code, but a visual editor allows you to create and modify sketches, automatically updating the code while you do that.


### Conclusion

So there it is. The criteria by which I judge CAD software. Unfortunately, I haven't found a single program yet that meets all of them. The search continues.

If you think I'm missing something, [please let me know](/contact). I'm always interested in learning more about the topics I write about.
