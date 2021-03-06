// *Br    0 :mgg       : mgg/F                                                  *
// *Entries :     3855 : Total  Size=      15958 bytes  File Size  =       1148 *
// *Baskets :        1 : Basket Size=      32000 bytes  Compression=  13.49     *
// *............................................................................*
// *Br    1 :mtot      : mtot/F                                                 *
// *Entries :     3855 : Total  Size=      15963 bytes  File Size  =        172 *
// *Baskets :        1 : Basket Size=      32000 bytes  Compression=  90.08     *
// *............................................................................*
// *Br    2 :evWeight  : evWeight/F                                             *
// *Entries :     3855 : Total  Size=      15983 bytes  File Size  =       1223 *
// *Baskets :        1 : Basket Size=      32000 bytes  Compression=  12.67     *
// *............................................................................*
// *Br    3 :Weight    : Weight/F                                               *
// *Entries :     3855 : Total  Size=      15973 bytes  File Size  =        967 *
// *Baskets :        1 : Basket Size=      32000 bytes  Compression=  16.02     *
// *............................................................................*
// *Br    4 :cut_based_ct : cut_based_ct/I                                      *
// *Entries :     3855 : Total  Size=      16003 bytes  File Size  =       2152 *
// *Baskets :        1 : Basket Size=      32000 bytes  Compression=   7.20     *
// *............................................................................*
// *Br    5 :BDT_ct    : BDT_ct/I                                               *
// *Entries :     3855 : Total  Size=      15973 bytes  File Size  =       2487 *
// *Baskets :        1 : Basket Size=      32000 bytes  Compression=   6.23     *
// *............................................................................*

{


  double mgg, mjj,evWeight, mtot, normWeight;
 int categories;

 evWeight = 1.0;
 normWeight = 1;


 string sInFile = "/uscms_data/d3/yongjie/8TeV/plots/CMSSW_5_2_4/src/53X_V3/DataVsMC/data/Categories/Bin1GeV/HZAnalysis/HWWqqqqZqqDataAndMC/Plots/data/dataPlotTight.root"; // input data root file

 cout << sInFile.c_str() << endl;
 TFile file0(sInFile.c_str(), "read");
 

 string sOutFile("MiniTrees/Data_HV/dijetHVtag_miniTree.root");
 TFile f1(sOutFile.c_str(), "recreate");
 f1.cd();
 
 TTree *TCVARS = new TTree("TCVARS", "VV selection");
 TCVARS->Branch("mgg",&mgg,"mgg/D");
 
 TCVARS->Branch("evWeight",&evWeight,"evWeight/D");
 TCVARS->Branch("normWeight",&normWeight,"normWeight/D");
 
 TCVARS->Branch("categories",&categories,"categories/I");
 
 
 
 for (int iCat = 0; iCat < 5; iCat++){
   TH1D* hMass = (TH1D*) file0.Get("DijetMassHwwPlot;1");
   if (iCat == 1) hMass = (TH1D*) file0.Get("DijetMassHwwPlotLowV;1");
   if (iCat == 2) hMass = (TH1D*) file0.Get("DijetMassHwwPlotLowH;1");
   if (iCat == 3) hMass = (TH1D*) file0.Get("DijetMassHbbPlotHighPurity;1");
   if (iCat == 4) hMass = (TH1D*) file0.Get("DijetMassHbbPlotLowPurity;1");
   

   TAxis* Axis =   hMass->GetXaxis();
   for (int i = 1 ; i < hMass->GetNbinsX()+1; i++){
     int N = abs(hMass->GetBinContent(i));
 
     if (i%10 == 0) cout << "i = " << i << "N = " << N << " binCenter = " << hMass->GetBinCenter(i) << endl;
     
     mgg = Axis->GetBinCenter(i);
     
     normWeight = N;
     categories = iCat;
     if (N > 1e-10 && mgg > 890 && mgg < 5000) TCVARS->Fill();
   }
 }

 

 TCVARS->Write();
 f1.Close();
 file0.Close();
 

}


