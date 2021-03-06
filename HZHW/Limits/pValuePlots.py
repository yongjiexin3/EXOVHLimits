import ROOT as rt
from ROOT import *
import os
import sys
from array import *

# define the plot canvas
def setStyle(c,histo):
    # canvas style
    rt.gStyle.SetOptStat(0)
    rt.gStyle.SetOptTitle(0)        
    
    c.SetTickx(1)
    c.SetTicky(1)
    
    c.SetRightMargin(0.14)
    c.SetTopMargin(0.14)
    c.SetLeftMargin(0.14)
    c.SetBottomMargin(0.14)
    
    # set x axis
    histo.GetXaxis().SetLabelFont(42)
    histo.GetXaxis().SetLabelSize(0.04)
    histo.GetXaxis().SetTitleFont(42)
    histo.GetXaxis().SetTitleSize(0.05)
    histo.GetXaxis().SetTitleOffset(1.2)
    histo.GetXaxis().CenterTitle(True)

    # set y axis
    histo.GetYaxis().SetLabelFont(42)
    histo.GetYaxis().SetLabelSize(0.04)
    histo.GetYaxis().SetTitleFont(42)
    histo.GetYaxis().SetTitleSize(0.05)
    histo.GetYaxis().SetTitleOffset(1.2)
    histo.GetYaxis().CenterTitle(True)

def PlotPValue(filePVALNAME, label):

    filePVAL = open(filePVALNAME)
    pvalPairs = eval(filePVAL.readline())

    mass = []
    pval = []

    for entry in pvalPairs:
        mass.append(entry[0]/1000.)
        pval.append(entry[1])

    pvalGraph = rt.TGraph(len(mass), array('d',mass), array('d',pval))
    c1 = rt.TCanvas("c1","c1", 300, 300)
    c1.SetLogy()
    pvalGraph.Draw("AL")
    htemp = pvalGraph.GetHistogram()
    c1.SetLogy()
    setStyle(c1,htemp)
    htemp.GetXaxis().SetTitle("Resonance Mass (TeV)")
    htemp.GetYaxis().SetTitle("p-value")
    htemp.SetMinimum(1E-5)
    htemp.SetMaximum(1)

    # sigmas
    sigmas = [0.8413447,0.9772499,0.9986501,0.9999683,0.9999997]
    line = rt.TLine()
    line.SetLineColor(2)
    for sigma in sigmas:
        line.DrawLine(htemp.GetXaxis().GetXmin(), 1-sigma,htemp.GetXaxis().GetXmax(), 1-sigma)
        line.Draw("SAME")
        pvalGraph.Draw("LSAME")
    text = []
    for i in range(1,6):
        text.append(rt.TLatex(htemp.GetXaxis().GetXmax()*0.92, (1-sigmas[i-1])*1.10,"%i #sigma" %i))
        #text = rt.TLatex(1800.,0.001,"%i #sigma" %i)
        #text.SetNDC()
        #text.SetTextFont(42)
        #text.SetTextSize(0.038)
        text[i-1].SetTextColor(2)
        text[i-1].Draw("SAME")

    banner = TLatex(0.27,0.93,"CMS Preliminary, 19.7 fb^{-1}, #sqrt{s} = 8TeV");
    banner.SetNDC()
    banner.SetTextSize(0.04)
    banner.Draw();  

    c1.Update()
    c1.SaveAs("pvalue_%s.pdf" %label)

def PlotMu(muFILENAME, label):
    
    fileMU = open(muFILENAME)
    muPairs = eval(fileMU.readline())

    mass = []
    mu = []
    muplus = []
    muminus = []
    for entry in muPairs:
        mass.append(entry[0]/1000.)
        mu.append(entry[1])
        muminus.append(entry[2]+entry[1])
        muplus.append(entry[3]+entry[1])
        if entry[1]<-150:
	    mu[-1]=-150
	    muminus[-1]=-150
	    if muplus[-1]<-150:
	       muplus[-1]=150
        if entry[1]>150:
	    mu[-1]=150
	    muplus[-1]=150
	    if muminus[-1]>150:
	       muminus[-1]=-150

    #nominal
    muGraph = rt.TGraph(len(mass), array('d',mass), array('d',mu))

    # green band
    muBAND = []
    massBAND = []
    for i in range(0, len(mass)):
        muBAND.append(muplus[i])
        massBAND.append(mass[i])
    for i in range(1, len(mass)+1):
        muBAND.append(muminus[len(mass)-i])
        massBAND.append(mass[len(mass)-i])

    print massBAND
    print muBAND

    muGraphBAND = rt.TGraph(len(massBAND), array('d',massBAND), array('d',muBAND))
    c1 = rt.TCanvas("c1","c1", 300, 300)
    muGraphBAND.SetFillStyle(1001)
    muGraphBAND.SetFillColor(rt.kGreen)
    muGraphBAND.Draw("AF")
    htemp = muGraphBAND.GetHistogram()
    setStyle(c1,htemp)
    htemp.GetXaxis().SetTitle("Resonance mass (TeV)")
    htemp.GetYaxis().SetTitle("Best-fit #sigma #times BR(X #rightarrow "+label.split("_")[0].replace("RS1","").replace("Bulk","")+") (pb)")
    htemp.SetMinimum(-150)
    htemp.SetMaximum(150)
    muGraph.Draw("PLSAME")
    l=rt.TLine(1000,0,2300,0)
    l.SetLineColor(2)
    l.Draw("same")

    c1.Update()
    c1.SaveAs("mu_%s.pdf" %label)

if __name__ == '__main__':

  channels=["RS1WW","RS1ZZ","WZ","qW","qZ","BulkWW","BulkZZ"]

  for chan in channels:
    print "chan =",chan
    if "q" in chan:
       cat="qV"
    elif "Bulk" in chan:
       cat="VV"
    else:
       cat="VV"

    HPplots="CMS_jj_"+chan+"_8TeV_CMS_jj_"+cat+"HP_pvalue.txt"
    LPplots="CMS_jj_"+chan+"_8TeV_CMS_jj_"+cat+"LP_pvalue.txt"
    combinedplots="CMS_jj_"+chan+"_8TeV_CMS_jj_"+cat+"_pvalue.txt"

    PlotPValue(HPplots,chan+"_high_purity")
    PlotPValue(LPplots,chan+"_low_purity")
    PlotPValue(combinedplots,chan+"_combined")
