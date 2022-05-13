+++
title = "Benefits of Open Source"
date  = 2021-11-04

[extra]
intro = """
Open source products, whether software or hardware, provide a lot of benefits. Both to users and their creators. In this note, I'm going to examine what those benefits are.
"""
+++

### What is Open Source?

So, what actually is open source? A good place to start answering that question, is to understand what "source" is. The "source" in open source refers to a representation of a product, that is the preferred representation for modifying that product.

In the case of software, this is the source code, written in a programming language. For hardware, it is usually CAD model and the associated documentation. *Open* source then means, that this source is publicly available.

In practical terms, this means that anyone with the necessary expertise can inspect and change the product. That is the core of open source.


### Licenses

Having source publicly available can still mean lots of different things. If you can see the source, but aren't allowed to modify or redistribute it, then its use is limited.

In its original sense, open source means that the source is available under a license that meets certain criteria, like allowing everyone to change and redistribute the source. For software, these criteria are defined in {{ ext_link(text="The Open Source Definition", url="https://opensource.org/osd") }}. For hardware, there's the {{ ext_link(text="Open Source Hardware Definition", url="https://www.oshwa.org/definition/") }}.

<aside>

I've been observing that the meaning of the term "open source" has shifted somewhat in popular use. Many people seem to use it to mean that the source is available to be read, regardless of its license.

I think this is a regrettable development, as it either leads to a loss of precision when speaking about this topic, or an increase in verbosity. When I talk about "open source", I specifically refer to the original meaning.
</aside>

Many licenses exist that meet these criteria. Most of them fall into one of two camps: Permissive or copyleft.

A permissive license allows many freedoms in changing and distributing the source, while making little to no requirements. A copyleft license only allows the source to be redistributed under the same license, which can complicate or outright prevent commercial use.

Personally, I'm a proponent of permissive licenses.


### Benefits for Users

Now that we know what the term means, let's check out some of the benefits we get from open source products.

#### Accessibility

In the case of software, open source means free of charge, at least in practical terms. This means you can test a solution thoroughly, without committing any resources beyond that.

Always make sure that your use of open source software is sustainable though! If a piece of software is critical to you, please check if the developers accept donations, or whether you can support them in other ways.

And while hardware is usually still associated with a cost, you have the ability to inspect it closely before committing to a purchase.

#### Simplicity

While the open source product itself might not be simple, there is at least no complicated end-user license. You don't have to worry about how many seats are left on your license, and there aren't any intrusive technical measures to enforce such a license.

#### Transparency

You can actually look at how the product works, and fully understand what it does. If you're having a problem, looking under the hood can be a great help in solving it.

Even if you're not capable of understanding the product yourself, others certainly are. You can benefit from their knowledge, by reading about how they solved the problem you're having, or by just hiring them to solve it for you.

#### Autonomy

With open source, there is no vendor lock-in. If paid service for a product gets worse, too expensive, or both, you can find someone else to provide that same service for you. If the creator of a product totally mismanages it, anyone can create their own version and continue the product's development under a different name.

#### Adaptability

You can adapt a product to your own needs, without relying on the vendor to add the features you need. By yourself, if you have the necessary skills, or by hiring someone to do it for you.

#### Decentralized Authority

Improvements aren't dependent on a single party recognizing their worth. If a product's creator is not convinced by an idea, someone else can prototype it. This also means that a whole ecosystem can develop around a product, with add-ons, extensions, or specialized versions, giving users more choice.


### Interlude: Open Source Projects

By itself, a product being open source already confers a number of benefits, as presented above. Creators can take things farther, by developing the product in an open and collaborative manner, as a true open source project.

What does this mean specifically?

- That the product is developed in the open. Everyone can see its current state and follow the latest development.
- That everyone can participate in its development. This doesn't mean you'll just let anyone make any changes they want, but everyone should be able to propose a change for inclusion into the product.
- That decisions are made in the open. Everyone can weigh in with their own perspective, and it's possible to later retrace how a decision was made.

This unlocks a number of additional benefits.


### Benefits for Creators

Not only users can benefit from open source products, their creators can too.

#### All User Benefits Are Your Benefits

Even if it doesn't seem like it on the surface, all those user benefits are your benefits too, if you market them effectively as the unique selling points they are. A customer concerned about vendor lock-in might be happy to spend money on your open source product, even if a competitor is technically superior.

#### Better Feedback

Users having more insight into how a product works often translates into better feedback. Users can dig into a problem and trace it down to a root cause before reporting it to you.

#### Contributions

If you accept contributions from the public, people will make improvements to the product and send them your way.

This can take the form of a feature that improves the product, but that you wouldn't have otherwise added yourself (possibly widening the market appeal). Or a fix for a problem that you would have had a hard time tracking down on your own. Or something you would have done anyway, but can now get for much cheaper.

#### Rich Ecosystem

If your product is developed as a true open source project, as outlined above, it has the potential to grow into something bigger than you could have done by yourself.

Third parties have an easier time creating complementary products that can make your product that much more attractive to customers. Maybe another company will join the development of your product and sell their own version, giving both of you a share in something much bigger than either of you could have built on your own.

This isn't without its challenges, of course. But as long as the open source project itself is healthy, it usually is in every participant's best interest to share as much with the project as possible. Otherwise, they cut themselves off from the constant stream of fixes and improvements that a healthy open source project accumulates constantly.


### It's Not All Or Nothing

If that previous section made you nervous, believe me, I understand. Doing open source development in a sustainable way, while capturing the benefits, can be quite a balancing act.

However, it's important to understand that this is not an all-or-nothing proposition. You can open source some parts, while keeping others proprietary. 

Maybe the core of your product can be open source, while you sell proprietary extensions and add-ons to customers with more advanced needs. Or you keep most of your product private, while open-sourcing only some parts, where you hope to encourage third-party development.

It is definitely not an easy thing, but the benefits are real. Maybe you can find a way that works for you.

<aside>

Are you considering open sourcing (parts of) your hardware or software product? Or maybe you are interested in the concept, but can't see how it could work for you?

In any case, [please get in touch](/contact)! It's a topic I love to talk about, and I'm very interested in learning more about how people think about this.
</aside>
