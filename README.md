# EXOVHLimits

This is the package to calculate the limits for Analysis EXO-14-009,  X -> HV, H->bb , V -> qq
Or X->HV,  H->WW->qqqq,  V-> qq

In this package,
(1)  root -b -q MiniTreeProducer.C  will generate the tree for data
(2) root -b -q MiniTreeSignalProducer.C will generate the tree for signals
(3) ./runR2J.sh will generate the data cards.   
    (3a) the C file for this is  R2JJFitterHV.C
(4) ./combine.sh will combine specific datacards together and put the resulting combined datacards in datacarsCom folder
(5) ./run.sh will generate the limits. 

Note:  the data and signal root files sits in other folder on cmslpc not lxplus. 
and step (3) will need these root files to calculate the data and signal yield. 
