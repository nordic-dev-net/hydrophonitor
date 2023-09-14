#!/bin/sh
# Enable capture and set input volume to 100% on all Behringer Uphoria 404HD inputs
set -ex
CARD=$(aplay -l | grep "USB Audio" | sed -e 's/^card\ \([0-9]\+\).*/\1/')
amixer -c $CARD cset "name=Mic Capture Switch,index=0" 1,1,1,1
amixer -c $CARD cset "name=Mic Capture Switch,index=1" 1
amixer -c $CARD cset "name=Mic Capture Volume,index=0" 127,127,127,127
amixer -c $CARD cset "name=Mic Capture Volume,index=1" 127
