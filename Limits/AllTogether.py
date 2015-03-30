import ROOT as rt
from ROOT import *

withAcceptance=False
unblind=True
number_of_mc_events=20000.

gROOT.Reset()
gROOT.SetStyle("Plain")
gStyle.SetOptStat(0)
gStyle.SetOptFit(0)
gStyle.SetTitleOffset(1.2,"Y")
gStyle.SetPadLeftMargin(0.18)
gStyle.SetPadBottomMargin(0.15)
gStyle.SetPadTopMargin(0.08)
gStyle.SetPadRightMargin(0.05)
gStyle.SetMarkerSize(0.5)
gStyle.SetHistLineWidth(1)
gStyle.SetStatFontSize(0.020)
gStyle.SetTitleSize(0.06, "XYZ")
gStyle.SetLabelSize(0.05, "XYZ")
gStyle.SetNdivisions(510, "XYZ")
gStyle.SetLegendBorderSize(0)

def Plot(files, label, obs):

    radmasses = [ m*0.1 for m in range(10,26+1) ]
    #for f in files:
    #    radmasses.append(float(f.replace("CMS_jj_","").split("_")[0])/1000.)
    #print radmasses

    #all the limits already have Higgs decay branching ratio taken into account
    #so here we only have to correct the Z decay branching ratio
    efficiencies={}
    for mass in radmasses:
	if "Z" in label :
	    efficiencies[mass]=0.01/0.699 # assume 10/fb signal cross section, take Z decay branching ratio into account
	if "W" in label :
            efficiencies[mass]=0.01/0.676 # assume 10/fb signal cross section, take W decay branching ratio into account

    fChain = []
    for onefile in files:
        print onefile
        fileIN = rt.TFile.Open(onefile)
        fChain.append(fileIN.Get("limit;1"))  

        rt.gROOT.ProcessLine("struct limit_t {Double_t limit;};")
        from ROOT import limit_t
        limit_branch = limit_t()

        for j in range(0,len(fChain)):
            chain = fChain[j]
            chain.SetBranchAddress("limit", rt.AddressOf(limit_branch,'limit'))

    rad = []
    for j in range(0,len(fChain)):
        chain = fChain[j]
        thisrad = []
        for  i in range(0,6):
            chain.GetTree().GetEntry(i)
            thisrad.append(limit_branch.limit)
            #print "limit = %f" %limit_branch.limit
        #print thisrad
        rad.append(thisrad)


    # we do a plot r*MR
   # mg = rt.TMultiGraph()
   # mg.SetTitle("X -> ZZ")
   # c1 = rt.TCanvas("c1","A Simple Graph Example",200,10,600,600)
    x = []
    yobs = []
    y2up = []
    y1up = []
    y1down = []
    y2down = []
    ymean = []

    for i in range(0,len(fChain)):
        y2up.append(rad[i][0]*efficiencies[radmasses[j]])
        y1up.append(rad[i][1]*efficiencies[radmasses[j]])
        ymean.append(rad[i][2]*efficiencies[radmasses[j]])
        y1down.append(rad[i][3]*efficiencies[radmasses[j]])
        y2down.append(rad[i][4]*efficiencies[radmasses[j]])
        yobs.append(rad[i][5]*efficiencies[radmasses[j]])

    grobs = rt.TGraphErrors(1)
    #grobs.SetMarkerStyle(rt.kFullDotLarge)
    grobs.SetMarkerStyle(24)
    grobs.SetMarkerSize(1)
    grobs.SetLineColor(rt.kRed)
    grobs.SetLineWidth(3)
    gr2up = rt.TGraphErrors(1)
    gr2up.SetMarkerColor(0)
    gr1up = rt.TGraphErrors(1)
    gr1up.SetMarkerColor(0)
    grmean = rt.TGraphErrors(1)
    grmean.SetLineColor(1)
    grmean.SetLineWidth(2)
    grmean.SetLineStyle(3)
    gr1down = rt.TGraphErrors(1)
    gr1down.SetMarkerColor(0)
    gr2down = rt.TGraphErrors(1)
    gr2down.SetMarkerColor(0)
  
    for j in range(0,len(fChain)):
        grobs.SetPoint(j, radmasses[j], yobs[j])
        gr2up.SetPoint(j, radmasses[j], y2up[j])
        gr1up.SetPoint(j, radmasses[j], y1up[j])
        grmean.SetPoint(j, radmasses[j], ymean[j])
        gr1down.SetPoint(j, radmasses[j], y1down[j])    
        gr2down.SetPoint(j, radmasses[j], y2down[j])
        #print " observed %f %f" %(radmasses[j],yobs[j])
    return grobs, grmean    


if __name__ == '__main__':

#  channels=["RS1WW","RS1ZZ","WZ","qW","qZ","BulkWW","BulkZZ"]
#  channels=["HbbZqqHPLP", "HbbWqqHPLP", "HwwZqqHPLPHV", "HwwWqqHPLPHV", "HbbWqqHwwHPLPHV", "HbbZqqHwwHPLPHV", "HWqq", "HZqq"] 
  channels=[ "HWqq", "HZqq", "HbbZqqHPLP", "HbbWqqHPLP", "HwwZqqHPLPHV", "HwwWqqHPLPHV", "HbbWqqHwwHPLPHV", "HbbZqqHwwHPLPHV"] 
  #channels=[ "HZqq"] 

  mg = rt.TMultiGraph()
  mg.SetTitle("X -> ZZ")
  c1 = rt.TCanvas("c1","A Simple Graph Example",200,10,600,600)
  c1.SetGrid()

  listPlot = []
  for chan in channels:
    print "chan =",chan
    masses = [ m*100 for m in range(10,26+1)]
    plots = []
    for mass in masses :
   	plots+=[ "higgsCombine" + chan +  ".Asymptotic.mH" + str(mass) + ".root" ]
  
    (a,b) = Plot( plots, chan, unblind )  
    if 'W' in chan :
	a.SetLineColor( kBlue )
	b.SetLineColor( kBlue )
    if 'Z' in chan :
	a.SetLineColor( kRed )
	b.SetLineColor( kRed )
    if chan=="HWqq" or chan=="HZqq" :
	a.SetMarkerStyle( 24 )
    if chan=="HbbWqqHPLP" or chan=="HbbZqqHPLP" :
	a.SetMarkerStyle( 20 )
    if chan=="HwwWqqHPLPHV" or chan=="HwwZqqHPLPHV" :
	a.SetMarkerStyle( 22 )
    if chan=="HbbWqqHwwHPLPHV" or chan=="HbbZqqHwwHPLPHV" :
	a.SetMarkerStyle( 30 )

    mg.Add( a, "PL" )
    mg.Add( b, "L" )
      
    listPlot.append(a)
#    listPlot.append(b)

  gtheory = rt.TGraphErrors(1)
  gtheory.SetLineColor(rt.kBlue)
  gtheory.SetLineWidth(4)
  ftheory=open("/uscms_data/d3/yongjie/HiggsCls/CMSSW_6_1_1/src/HbbZqq/CombineOfficial/Limits/HZcrossSection.txt")
  j=0
  glogtheory = rt.TGraphErrors(1)
  for line in ftheory.readlines() :
      newline = line.strip().split()
      print float(newline[1])*float(newline[2])/1000
      gtheory.SetPoint(j, float(newline[0])/1000, float(newline[1])*float(newline[2])/1000 )
      glogtheory.SetPoint(j, float(newline[0])/1000, float(newline[1])*float(newline[2])/1000   )
      j = j + 1

  mg.Add(gtheory,"L")
  gtheory.Draw("L")
  gtheory.SetLineColor( 1 )
  gtheory.SetLineWidth( 3 )
  gtheory.SetLineStyle( 9 )

  leg = rt.TLegend(0.45,0.65,0.86,0.89)
  #leg.SetFillColor(rt.kWhite)
  leg.SetFillStyle(0)
  leg.SetTextSize(0.04)
  leg.SetBorderSize(0)
  leg.AddEntry( listPlot[0], 'All combined', "PL")
  leg.AddEntry( listPlot[2], 'H #rightarrow bb', "PL")
  leg.AddEntry( listPlot[4], 'H #rightarrow WW*', 'PL')
  leg.AddEntry( listPlot[6], 'H #rightarrow bb in H #rightarrow WW* ', "PL")
  leg.AddEntry( gtheory, "V'to HV theory", "L")



  c1.SetLogy(1)
  mg.SetTitle( "" )    
  mg.Draw("AP")
  mg.GetXaxis().SetTitle("Resonance mass (TeV)")

#  channels=["HbbZqqHPLP", "HbbWqqHPLP", "HwwZqqHPLPHV", "HwwWqqHPLPHV", "HbbWqqHwwHPLPHV", "HbbZqqHwwHPLPHV", "HWqq", "HZqq"]
  mg.GetYaxis().SetTitle("#sigma #times B(V'->HV) (pb)" )

  mg.GetYaxis().SetTitleSize(0.06)
  mg.GetXaxis().SetTitleSize(0.06)
  mg.GetXaxis().SetLabelSize(0.045)
  mg.GetYaxis().SetLabelSize(0.045)
  mg.GetYaxis().SetRangeUser(0.004,4)
  mg.GetYaxis().SetTitleOffset(1.4)
  mg.GetYaxis().CenterTitle(True)
  mg.GetXaxis().SetTitleOffset(1.1)
  mg.GetXaxis().CenterTitle(True)
  mg.GetXaxis().SetNdivisions(508)

  mg.GetXaxis().SetLimits(0.9,2.7)
  leg.Draw()

  c1.SaveAs( "AllOn1Plot.eps" )
  c1.SaveAs( "AllOn1Plot.pdf" )



