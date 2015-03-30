#!/bin/sh

for i in `seq 1000 50 2600`
do
    #combine all channels for HZ
    combineCards.py datacards/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbHP.txt datacards/CMS_jj_HbbZqq_$i\_8TeV_CMS_jj_HbbLP.txt datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HwwZqq_$i\_8TeV_CMS_jj_HwwLPH.txt > datacardsAll/CMS_jj_HZqq_$i\_8TeV_CMS_jj_HZCombined.txt &

   #combine all channels for HW
   combineCards.py datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbHP.txt datacards/CMS_jj_HbbWqq_$i\_8TeV_CMS_jj_HbbLP.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwHP.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPV.txt datacards/CMS_jj_HwwWqq_$i\_8TeV_CMS_jj_HwwLPH.txt > datacardsAll/CMS_jj_HWqq_$i\_8TeV_CMS_jj_HWCombined.txt &

done
