#!/bin/sh

#for i in `seq 1000 100 2600`
for i in `seq 1000 50 2600`
#for i in `seq 1900 100 2600`
#for i in 2150
do
    #root -b -q 'R2JJFitterHV.C( '$i', "", 1)' &
    #root -b -q 'R2JJFitterHV.C( '$i', "", 0)' & 
    root -b -q 'R2JJFitterHV.C( '$i', "", 1)'   & 
#> log2 &
    #root -b -q 'R2JJFitterHV.C( '$i', "", 1)' > log2 &
done


#root -b -q 'R2JJFitterHV.C(1000, "", 0)' &
#root -b -q 'R2JJFitterHV.C(1000, "", 1)' &
