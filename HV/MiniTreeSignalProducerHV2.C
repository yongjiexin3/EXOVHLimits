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

 for (int iSample = 0; iSample < 6; iSample++){
  
   string pathWay( "/uscms_data/d3/yongjie/8TeV/plots/CMSSW_5_2_4/src/53X_V3/DataVsMC/data/Categories/Bin1GeV/HZAnalysis/");
   //HwwZqq input files 
   string inFile("HWWqqqqZqqDataAndMC/Signal/NewSignal/HwwSignalWithB/PileUpBScale/PileUpBScale");
   string outName("HwwZqq");
   // HwwWqq input files
   if (iSample == 1 )  {
	inFile =  string("HWWqqqqZqqDataAndMC/Signal/NewSignal/HwwSignalWithB/PileUpBScale/HwwWqqPileUpBScale");
	outName = string("HwwWqq");
   }
   //HbbZqq pass Hww tagger
   if (iSample == 2 ) {
	 inFile = string("HWWqqqqZqqDataAndMC/Signal/NewSignal/HbbSignalPassHww/PileUpBScale/PileUpBScale");
	 outName = string("HbbZqqHww");
   }
   //HbbWqq pass Hww tagger
   if (iSample == 3) {
	inFile = string("HWWqqqqZqqDataAndMC/Signal/NewSignal/HbbSignalPassHww/PileUpBScale/HbbWqqPileUpBScale");
	outName = string("HbbWqqHww");
    }
   //HbbZqq pass Hbb
   if (iSample == 4) {
 	inFile = string("HbbZqq/Signal/PlotsGeneration/PileUpBScale/PileUpBScale");
	outName = string("HbbZqq");
    }
   //HbbWqq pass Hbb
   if (iSample == 5) {
	inFile = string("HbbZqq/Signal/PlotsGeneration/PileUpBScale/HbbWqqPileUpBScale");
	outName = string("HbbWqq");
    }
   for (int iMass = 0; iMass<17; iMass++){

     //string sInFile = "~hinzmann/public/forMaxime/" + inFile + "" + Form("OUT%d.root", 1000+iMass*100);
     string sInFile = pathWay + inFile + "" + Form("%dGeV.root", 1000+iMass*100);
     cout << sInFile.c_str() << endl;
     TFile file0(sInFile.c_str(), "read");


     string sOutFile = "MiniTrees/Signal_HV/" + outName + Form("OUT%d_miniTree.root", 1000+iMass*100);
     TFile f1(sOutFile.c_str(), "recreate");
     f1.cd();

     TTree *TCVARS = new TTree("TCVARS", "VV selection");
     TCVARS->Branch("mgg",&mgg,"mgg/D");

     TCVARS->Branch("evWeight",&evWeight,"evWeight/D");
     TCVARS->Branch("normWeight",&normWeight,"normWeight/D");
     
     TCVARS->Branch("categories",&categories,"categories/I");

  
     double dMass = 1000.+iMass*100.;


     if ( iSample < 4 ) {
         for (int iCat = 0; iCat < 5; iCat++){
	   TH1D* hMass = (TH1D*) file0.Get( "accPlot" );
           if (iCat == 0)  hMass = (TH1D*) file0.Get("DijetMassHighPuri;1");
           if (iCat == 1)  hMass = (TH1D*) file0.Get("DijetMassLowPuriV;1");
           if (iCat == 2)  hMass = (TH1D*) file0.Get("DijetMassLowPuriH;1");
           TAxis* Axis =   hMass->GetXaxis();
           for (int i = 1 ; i < hMass->GetNbinsX()+1; i++){
            // if (hMass->GetBinCenter(i) < dMass*0.7 || hMass->GetBinCenter(i) > dMass*1.3) continue;
               int N = abs(hMass->GetBinContent(i));
               if (i%1000 == 0) cout << "i = " << i << endl;

               mgg = Axis->GetBinCenter(i);

               for (int k = 0; k < N; k++) {
                   categories = iCat;
                   TCVARS->Fill();
               }
          }
        }

    }

    if ( iSample > 3 ) {
       for (int iCat = 0; iCat < 5; iCat++){
           TH1D* hMass = (TH1D*) file0.Get( "accPlot" );
           if (iCat == 3)  hMass = (TH1D*) file0.Get("DijetMassHighP;1");
           if (iCat == 4)  hMass = (TH1D*) file0.Get("DijetMassLowP;1");
           TAxis* Axis =   hMass->GetXaxis();
           for (int i = 1 ; i < hMass->GetNbinsX()+1; i++){
            // if (hMass->GetBinCenter(i) < dMass*0.7 || hMass->GetBinCenter(i) > dMass*1.3) continue;
               int N = abs(hMass->GetBinContent(i));
               if (i%1000 == 0) cout << "i = " << i << endl;

               mgg = Axis->GetBinCenter(i);

               for (int k = 0; k < N; k++) {
                   categories = iCat;
                   TCVARS->Fill();
               }
          }
        }


    }


     TCVARS->Write();
     f1.Close();
     file0.Close();
     
   }

 }


}


