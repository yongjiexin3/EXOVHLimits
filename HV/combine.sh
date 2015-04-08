#!/bin/sh

for i in `seq 1000 50 2600`
#for i in 1000
do

   #Combine HV channel together
   combineCards.py datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbHP.txt datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbLP.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPH.txt > datacardsAll/CMS_jj_HVqq_$i\_8TeV_CMS_jj_HVCombined.txt &

done
