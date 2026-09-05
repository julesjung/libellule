//
//  RootView.swift
//  Libellule
//
//  Created by Jules on 10/08/2026.
//

import SwiftUI

struct RootView: View {
    @State var appParameters: AppParameters?
    
    init(appParameters: AppParameters?) {
        self._appParameters = State(initialValue: appParameters)
    }
    
    var body: some View {
        if let appParameters {
            tabs(appParameters: appParameters)
        } else {
            LoginView(appParameters: $appParameters)
        }
        //        switch session.state {
        //        case .loggedOut:
        //            InstanceView()
        //        case .connecting, .authenticating:
        //            ProgressView()
        //        case .connected(let instance):
        //            LoginView(instance: instance)
        //        case .authenticated(let client):
        
        //                Tab("Devoirs", systemImage: "") {
        //                    HomeworkView(client: client)
        //                }
        //                Tab("Notes", systemImage: "graph.2d") {
        //                    GradesView(store: GradesStore(client: client))
        //                }
        //        case .failed(let error):
        //            ContentUnavailableView {
        //                Label("Erreur", systemImage: "wifi.exclamationmark")
        //            } description: {
        //                Text(error.localizedDescription)
        //            } actions: {
        //                Button("Retour à l'écran de connexion") {
        //                    session.state = .loggedOut
        //                }
        //                    .buttonStyle(.glassProminent)
        //                    .controlSize(.large)
        //                    .buttonSizing(.flexible)
        //            }
        //        }
    }
    
    @ViewBuilder
    func tabs(appParameters: AppParameters) -> some View {
        let datesRange: ClosedRange<Date> = {
            let startDate = DateFormatter.date.date(from: appParameters.startDate)!
            let endDate = DateFormatter.date.date(from: appParameters.endDate)!
            
            return startDate...endDate
        }()
        
        TabView {
            Tab("Emploi du temps", systemImage: "calendar") {
                TimetableView(datesRange: datesRange)
            }
        }
    }
}
