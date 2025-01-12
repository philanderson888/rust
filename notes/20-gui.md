# gui

## contents

- [gui](#gui)
  - [contents](#contents)
  - [introduction](#introduction)
  - [gtk](#gtk)
    - [versions](#versions)
    - [06](#06)
    - [07](#07)
    - [08](#08)
  - [druid](#druid)
  - [iced](#iced)
    - [iced reference](#iced-reference)
    - [iced terminology](#iced-terminology)
    - [elm architecture](#elm-architecture)

## introduction

we can use rust to build graphic applications 

there are several libraries which are available for this purpose

one is `gtk` and we have used this to build a crude graphics application

## gtk

### versions

### 06 

modifying css slightly

### 07

build two buttons not just one

### 08

using weak references to build the counters for the two buttons

## druid

## iced

we can use iced to build gui apps

### iced reference

https://book.iced.rs/

### iced terminology

- widgets interacted with by the user
- interactions are the events triggered
- state is underlying state of app

### elm architecture

elm is purely functional programming language

- model
- messages
- update logic - how messages change state (model)
- view logic - how the state changes the widgets

