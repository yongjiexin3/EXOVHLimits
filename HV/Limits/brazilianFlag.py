import ROOT as rt
from ROOT import *

withAcceptance=False
unblind=True
number_of_mc_events=30000.

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

    radmasses = []
    for f in files:
        radmasses.append(float(f.replace("CMS_jj_","").split("_")[0])/1000.)
    #print radmasses

    efficiencies={}
    for mass in radmasses:
	efficiencies[mass]=0.01 # assume 10/fb signal cross section

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
    mg = rt.TMultiGraph()
    mg.SetTitle("X -> ZZ")
    c1 = rt.TCanvas("c1","A Simple Graph Example",200,10,600,600)
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
    grobs.SetMarkerStyle(rt.kFullDotLarge)
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
    
    mg.Add(gr2up)#.Draw("same")
    mg.Add(gr1up)#.Draw("same")
    mg.Add(grmean,"L")#.Draw("same,AC*")
    mg.Add(gr1down)#.Draw("same,AC*")
    mg.Add(gr2down)#.Draw("same,AC*")
    if obs: mg.Add(grobs,"L")#.Draw("AC*")
 
    c1.SetLogy(1)
    mg.SetTitle("")
    mg.Draw("AP")
    mg.GetXaxis().SetTitle("Resonance mass (TeV)")
    if "qW" in label.split("_")[0] or "qZ" in label.split("_")[0]:
        resonance="q*"
    if "RS1" in label.split("_")[0]:
        resonance="G_{RS}"
    if "Bulk" in label.split("_")[0]:
        resonance="G_{Bulk}"
    if "WZ" in label.split("_")[0]:
        resonance="W'"
    if withAcceptance:
        mg.GetYaxis().SetTitle("#sigma #times B("+resonance+" #rightarrow "+label.split("_")[0].replace("RS1","").replace("Bulk","")+") #times A (pb)")
    else:
        mg.GetYaxis().SetTitle("#sigma #times B("+resonance+" #rightarrow "+label.split("_")[0].replace("RS1","").replace("Bulk","")+") (pb)")
    mg.GetYaxis().SetTitleSize(0.06)
    mg.GetXaxis().SetTitleSize(0.06)
    mg.GetXaxis().SetLabelSize(0.045)
    mg.GetYaxis().SetLabelSize(0.045)
    mg.GetYaxis().SetRangeUser(0.0004,4)
    mg.GetYaxis().SetTitleOffset(1.4)
    mg.GetYaxis().CenterTitle(True)
    mg.GetXaxis().SetTitleOffset(1.1)
    mg.GetXaxis().CenterTitle(True)
    mg.GetXaxis().SetNdivisions(508)

    if "qW" in label.split("_")[0] or "qZ" in label.split("_")[0]:
        mg.GetXaxis().SetLimits(0.9,4.1)
    else:
        mg.GetXaxis().SetLimits(0.9,3.1)

    # histo to shade
    n=len(fChain)

    grgreen = rt.TGraph(2*n)
    for i in range(0,n):
        grgreen.SetPoint(i,radmasses[i],y2up[i])
        grgreen.SetPoint(n+i,radmasses[n-i-1],y2down[n-i-1])

    grgreen.SetFillColor(rt.kGreen)
    grgreen.Draw("f") 


    gryellow = rt.TGraph(2*n)
    for i in range(0,n):
        gryellow.SetPoint(i,radmasses[i],y1up[i])
        gryellow.SetPoint(n+i,radmasses[n-i-1],y1down[n-i-1])

    gryellow.SetFillColor(rt.kYellow)
    gryellow.Draw("f,same") 

    grmean.Draw("L")
    if obs: grobs.Draw("L")

    gtheory = rt.TGraphErrors(1)
    gtheory.SetLineColor(rt.kBlue)
    gtheory.SetLineWidth(4)
    ftheory=open("signalcrosssections.txt")
    j=0
    glogtheory = rt.TGraphErrors(1)
    for lines in ftheory.readlines():
     for line in lines.split("\r"):
      if label.split("_")[0] in line and line.count("Bulk")==label.split("_")[0].count("Bulk"):
        split=line.split(":")
        gtheory.SetPoint(j, float(split[0][-4:])/1000., float(split[1]))
        glogtheory.SetPoint(j, float(split[0][-4:])/1000., log(float(split[1])))
	j+=1
    mg.Add(gtheory,"L")
    gtheory.Draw("L")
    if "qW" in label.split("_")[0]:
        ltheory="q* #rightarrow qW"
    elif "qZ" in label.split("_")[0]:
        ltheory="q* #rightarrow qZ"
    elif "BulkWW" in label.split("_")[0]:
        ltheory="G_{Bulk} #rightarrow WW (k/#bar{M}_{Pl}=0.2)"
    elif "RS1WW" in label.split("_")[0]:
        ltheory="G_{RS} #rightarrow WW (k/#bar{M}_{Pl}=0.1)"
    elif "BulkZZ" in label.split("_")[0]:
        ltheory="G_{Bulk} #rightarrow ZZ (k/#bar{M}_{Pl}=0.2)"
    elif "RS1ZZ" in label.split("_")[0]:
        ltheory="G_{RS} #rightarrow ZZ (k/#bar{M}_{Pl}=0.1)"
    elif "WZ" in label.split("_")[0]:
        ltheory="W' #rightarrow WZ"
    
    crossing=0
    for mass in range(int(radmasses[0]*1000.),int(radmasses[-1]*1000.)):
        if exp(glogtheory.Eval(mass/1000.))>grmean.Eval(mass/1000.) and crossing>=0:
	    print label,"exp crossing",mass
	    crossing=-1
        if exp(glogtheory.Eval(mass/1000.))<grmean.Eval(mass/1000.) and crossing<=0:
	    print label,"exp crossing",mass
	    crossing=1
    crossing=0
    for mass in range(int(radmasses[0]*1000.),int(radmasses[-1]*1000.)):
        if exp(glogtheory.Eval(mass/1000.))>grobs.Eval(mass/1000.) and crossing>=0:
	    print label,"obs crossing",mass
	    crossing=-1
        if exp(glogtheory.Eval(mass/1000.))<grobs.Eval(mass/1000.) and crossing<=0:
	    print label,"obs crossing",mass
	    crossing=1
    
    if "WW" in label.split("_")[0] or "ZZ" in label.split("_")[0]:
       leg = rt.TLegend(0.43,0.65,0.95,0.89)
       leg2 = rt.TLegend(0.43,0.65,0.95,0.89)
    else:
       leg = rt.TLegend(0.59,0.65,0.95,0.89)
       leg2 = rt.TLegend(0.59,0.65,0.95,0.89)
    leg.SetFillColor(rt.kWhite)
    leg.SetFillStyle(0)
    leg.SetTextSize(0.04)
    leg.SetBorderSize(0)
    leg2.SetFillColor(rt.kWhite)
    leg2.SetFillStyle(0)
    leg2.SetTextSize(0.04)
    leg2.SetBorderSize(0)

    if obs: leg.AddEntry(grobs, "Observed", "L")
    leg.AddEntry(gryellow, "Expected (68%)", "f")
    leg.AddEntry(grgreen, "Expected (95%)", "f")
    leg.AddEntry(gtheory, ltheory, "L")

    if obs: leg2.AddEntry(grobs, " ", "")
    leg2.AddEntry(grmean, " ", "L")
    leg2.AddEntry(grmean, " ", "L")
    leg2.AddEntry(gtheory, " ", "")

    leg.Draw()
    leg2.Draw("same")

    banner = TLatex(0.21,0.88,"CMS");
    banner.SetNDC()
    banner.SetTextSize(0.045)
    banner.Draw();  

    banner2 = TLatex(0.73,0.94,"19.7 fb^{-1} (8TeV)");
    banner2.SetNDC()
    banner2.SetTextSize(0.035)
    banner2.Draw();  
    banner2.SetTextFont(42);
    banner2.SetTextAlign(12)



    if withAcceptance:
        c1.SaveAs("brazilianFlag_acc_%s.root" %label)
        c1.SaveAs("brazilianFlag_acc_%s.pdf" %label)
    else:
        c1.SaveAs("brazilianFlag_%s.root" %label)
        c1.SaveAs("brazilianFlag_%s.pdf" %label)


if __name__ == '__main__':

  channels=["RS1WW","RS1ZZ","WZ","qW","qZ","BulkWW","BulkZZ"]

  for chan in channels:
    print "chan =",chan
    if "q" in chan:
       masses =[m*100 for m in range(10,39+1)]
       cat="qV"
    elif "Bulk" in chan:
       masses =[m*100 for m in range(10,29+1)]
       cat="VV"
    else:
       masses =[m*100 for m in range(10,29+1)]
       cat="VV"

    HPplots=[]
    LPplots=[]
    combinedplots=[]
    for mass in masses:
       HPplots+=["CMS_jj_"+str(mass)+"_"+chan+"_8TeV_CMS_jj_"+cat+"HP_asymptoticCLs.root"]
       LPplots+=["CMS_jj_"+str(mass)+"_"+chan+"_8TeV_CMS_jj_"+cat+"LP_asymptoticCLs.root"]
       combinedplots+=["CMS_jj_"+str(mass)+"_"+chan+"_8TeV_CMS_jj_"+cat+"_asymptoticCLs.root"]

    Plot(HPplots,chan+"_high_purity", unblind)
    Plot(LPplots,chan+"_low_purity", unblind)
    Plot(combinedplots,chan+"_combined", unblind)
