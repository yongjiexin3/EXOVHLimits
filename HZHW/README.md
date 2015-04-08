# EXOVHLimits

This is the package to calculate the limits for Analysis EXO-14-009,  X -> HV, H->bb , V -> qq
Or X->HV,  H->WW->qqqq,  V-> qq

In this package,
(1)  root -b -q MiniTreeProducer.C  will generate the tree for data

(2) root -b -q MiniTreeSignalProducer.C will generate the tree for signals

(3) ./runR2J.sh will generate the data cards.   
    (3a) the C file for this is  R2JJFitterHV.C

(4) ./combine.sh will combine specific datacards together and put the resulting combined datacards in datacardsAll folder

(5) ./run.sh will generate the limits. 

Note:  the data and signal root files sit in other folders on cmslpc not lxplus. 
and step (3) will need these root files to calculate the data and signal yield.

This package will give you the limits of HW and HZ channel. 



Folders :

datacards is the one having all the datacards for each specific channel in their low purity, and in their high purity

datacardsAll is the one having the datacards for HZ and HW, (combining all the relevant channels)

datacardsCom is the one for HbbWqq, HbbZqq, HwwWqq, HwwZqq,....  those specific channels, but combining
the low purity and high purity 

HbbVqqHwwworkspaces having the work space for HbbVqq passing Hww categories.

workspaces having the work space for the rest channels. 

Limits folder is the one having tools for making limit plots
