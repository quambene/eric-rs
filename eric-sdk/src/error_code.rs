#![allow(non_camel_case_types)]

pub enum ErrorCode {
    /**
     * [0] Verarbeitung fehlerfrei.
     */
    ERIC_OK = 0,

    /**
     * [610001001] Verarbeitung fehlerhaft, keine genaueren Informationen vorhanden. Details stehen ggf. im Logfile (eric.log).
     */
    ERIC_GLOBAL_UNKNOWN = 610001001,

    /**
     * [610001002] Fehler während der Plausibilitätsprüfung, Datensatz nicht plausibel. Zur Ermittlung der fehlgeschlagenen Plausibiltätsprüfungen muss der Rückgabepuffer (Parameter "rueckgabeXmlPuffer") ausgewertet werden.
     */
    ERIC_GLOBAL_PRUEF_FEHLER = 610001002,

    /**
     * [610001003] Hinweise während der Plausibilitätsprüfung, Datensatz ist aber plausibel. Zur Ermittlung der anzuzeigenden Hinweise muss der Rückgabepuffer (Parameter "rueckgabeXmlPuffer") ausgewertet werden.
     */
    ERIC_GLOBAL_HINWEISE = 610001003,

    /**
     * [610001007] Keine Klartextfehlermeldung vorhanden.
     */
    ERIC_GLOBAL_FEHLERMELDUNG_NICHT_VORHANDEN = 610001007,

    /*
     * [610001008] Für den übergebenen Wert sind keine Daten vorhanden.
     */
    ERIC_GLOBAL_KEINE_DATEN_VORHANDEN = 610001008,

    /**
     * [610001013] Es ist nicht genügend Arbeitsspeicher vorhanden.
     */
    ERIC_GLOBAL_NICHT_GENUEGEND_ARBEITSSPEICHER = 610001013,

    /**
     * [610001014] Datei nicht gefunden.
     */
    ERIC_GLOBAL_DATEI_NICHT_GEFUNDEN = 610001014,

    /**
     * [610001016] Für dieses Verfahren/diese Datenart ist eine Bearbeitung mit der angegebenen HerstellerID nicht erlaubt.
     */
    ERIC_GLOBAL_HERSTELLER_ID_NICHT_ERLAUBT = 610001016,

    /**
     * [610001017] Ungültiger Zustand.
     */
    ERIC_GLOBAL_ILLEGAL_STATE = 610001017,

    /**
     * [610001018] Die aufgerufene Funktion ist nicht erlaubt.
     */
    ERIC_GLOBAL_FUNKTION_NICHT_ERLAUBT = 610001018,

    /**
     * [610001019] Für dieses Verfahren/diese Datenart/diese Test-HerstellerID/diese ERiC-Einstellungen sind Echtfälle nicht erlaubt.
     */
    ERIC_GLOBAL_ECHTFALL_NICHT_ERLAUBT = 610001019,

    /**
     * [610001020] Der Versand von Echtfällen (= Fällen ohne gesetzten Testmerker) ist mit einer BETA-Version nicht möglich.
     */
    ERIC_GLOBAL_NO_VERSAND_IN_BETA_VERSION = 610001020,

    /**
     * [610001025] Der übergebene Testmerker ist für das angegebene Verfahren nicht zulässig.
     */
    ERIC_GLOBAL_TESTMERKER_UNGUELTIG = 610001025,

    /**
     * [610001026] Der zu versendende Datensatz ist zu groß.
     */
    ERIC_GLOBAL_DATENSATZ_ZU_GROSS = 610001026,

    /**
     * [610001027] Der Verschlüsselungsparameter darf nur bei authentifiziertem Versand angegeben werden.
     */
    ERIC_GLOBAL_VERSCHLUESSELUNGS_PARAMETER_NICHT_ERLAUBT = 610001027,

    /**
     * [610001028] Bei der angegebenen Versandart sind nur Portal-Zertifikate erlaubt.
     */
    ERIC_GLOBAL_NUR_PORTALZERTIFIKAT_ERLAUBT = 610001028,

    /**
     * [610001029] Für die übermittelte Datenart ist die Angabe eines Abrufcodes nicht zulässig, daher muss für den Abrufcode der Wert NULL übergeben werden.
     */
    ERIC_GLOBAL_ABRUFCODE_NICHT_ERLAUBT = 610001029,

    /**
     * [610001030] Es ist ein Fehler bei der Umwandlung nach XML aufgetreten.
     */
    ERIC_GLOBAL_ERROR_XML_CREATE = 610001030,

    /**
     * [610001031] Die Größe des Textpuffers kann nicht verändert werden.
     */
    ERIC_GLOBAL_TEXTPUFFERGROESSE_FIX = 610001031,

    /**
     * [610001032] Interner Fehler aufgetreten. Details stehen ggf. im Logfile (eric.log).
     */
    ERIC_GLOBAL_INTERNER_FEHLER = 610001032,

    /**
     * [610001033] Bei einer arithmetischen Operation ist ein Fehler aufgetreten. Details stehen im Logile (eric.log)
     */
    ERIC_GLOBAL_ARITHMETIKFEHLER = 610001033,

    /**
     * [610001034] Ungültige Steuernummer.
     */
    ERIC_GLOBAL_STEUERNUMMER_UNGUELTIG = 610001034,

    /**
     * [610001035] Ungültige Steuernummer: Es werden 13 Stellen erwartet.
     */
    ERIC_GLOBAL_STEUERNUMMER_FALSCHE_LAENGE = 610001035,

    /**
     * [610001036] Ungültige Steuernummer: Es werden nur Ziffern erwartet.
     */
    ERIC_GLOBAL_STEUERNUMMER_NICHT_NUMERISCH = 610001036,

    /**
     * [610001037] Ungültige Landesnummer.
     */
    ERIC_GLOBAL_LANDESNUMMER_UNBEKANNT = 610001037,

    /**
     * [610001038] Ungültige Bundesfinanzamtsnummer.
     */
    ERIC_GLOBAL_BUFANR_UNBEKANNT = 610001038,

    /**
     * [610001039] Ungültige Bundesfinanzamtsnummer.
     */
    ERIC_GLOBAL_LANDESNUMMER_BUFANR = 610001039,

    /**
     * [610001040] Ein Puffer-Handle wurde mehrfach übergeben.
     */
    ERIC_GLOBAL_PUFFER_ZUGRIFFSKONFLIKT = 610001040,

    /**
     * [610001041] Es wurde versucht, einen Puffer über die maximal mögliche Länge hinaus zu beschreiben.
     */
    ERIC_GLOBAL_PUFFER_UEBERLAUF = 610001041,

    /**
     * [610001042] Die übergebene Datenartversion ist unbekannt. Beachten Sie bitte, dass die Datenartversion case-sensitive ist.
     */
    ERIC_GLOBAL_DATENARTVERSION_UNBEKANNT = 610001042,

    /**
     * [610001044] Die übergebene Datenartversion passt nicht zum Eingangs-XML. Details stehen ggf. im Logfile (eric.log).
     */
    ERIC_GLOBAL_DATENARTVERSION_XML_INKONSISTENT = 610001044,

    /**
     * [610001045] Das Plugin 'commonData' konnte nicht geladen werden oder bietet einen benötigten Service nicht an. Details stehen im Logfile (eric.log).
     */
    ERIC_GLOBAL_COMMONDATA_NICHT_VERFUEGBAR = 610001045,

    /**
     * [610001046] Beim Schreiben in die Protokolldatei ist eine Ausnahme aufgetreten.
     */
    ERIC_GLOBAL_LOG_EXCEPTION = 610001046,

    /**
     * [610001047] Für diese Datenart darf im TransferHeader kein TransportSchluessel angegeben werden.
     */
    ERIC_GLOBAL_TRANSPORTSCHLUESSEL_NICHT_ERLAUBT = 610001047,

    /**
     * [610001048] Der übergebene öffentliche Schlüssel kann nicht eingelesen werden.
     */
    ERIC_GLOBAL_OEFFENTLICHER_SCHLUESSEL_UNGUELTIG = 610001048,

    /**
     * [610001049] Der Typ des im TransferHeader angegebenen Transportschlüssels ist für diese Datenart nicht erlaubt.
     */
    ERIC_GLOBAL_TRANSPORTSCHLUESSEL_TYP_FALSCH = 610001049,

    /**
     * [610001050] Das übergebene Puffer-Handle wurde nicht mit der vorliegenden Instanz erzeugt.
     */
    ERIC_GLOBAL_PUFFER_UNGLEICHER_INSTANZ = 610001050,

    /**
     * [610001051] Das Element "Vorsatz" enthält ungültige Werte, Details stehen im Logfile (eric.log).
     */
    ERIC_GLOBAL_VORSATZ_UNGUELTIG = 610001051,

    /**
     * [610001053] Auf eine Datei konnte nicht in gewünschter Weise zugegriffen werden. Details stehen im Logile (eric.log)
     */
    ERIC_GLOBAL_DATEIZUGRIFF_VERWEIGERT = 610001053,

    /**
     * [610001080] Die übergebene Instanz ist gleich NULL oder bereits freigegeben worden.
     */
    ERIC_GLOBAL_UNGUELTIGE_INSTANZ = 610001080,

    /**
     * [610001081] Der Singlethread-ERiC wurde nicht mit EricInitialisiere initialisiert.
     */
    ERIC_GLOBAL_NICHT_INITIALISIERT = 610001081,

    /**
     * [610001082] Der Singlethread-ERiC wurde bereits mit EricInitialisiere initialisiert.
     */
    ERIC_GLOBAL_MEHRFACHE_INITIALISIERUNG = 610001082,

    /**
     * [610001083] Der angeforderte ERiC-Instanz konnte nicht erstellt werden. Details stehen ggf. im Logfile (eric.log).
     */
    ERIC_GLOBAL_FEHLER_INITIALISIERUNG = 610001083,

    /**
     * [610001102] Unbekannter Parameterfehler.
     */
    ERIC_GLOBAL_UNKNOWN_PARAMETER_ERROR = 610001102,

    /**
     * [610001108] Defekter Nutzdatensatz.
     */
    ERIC_GLOBAL_CHECK_CORRUPTED_NDS = 610001108,

    /**
     * [610001206] Verschlüsselter/authentifizierter Versand gewünscht, aber keine notwendigen Verschlüsselungsparameter angegeben.
     */
    ERIC_GLOBAL_VERSCHLUESSELUNGS_PARAMETER_NICHT_ANGEGEBEN = 610001206,

    /**
     * [610001209] Es ist mehr als ein Versandflag angegeben.
     */
    ERIC_GLOBAL_SEND_FLAG_MEHR_ALS_EINES = 610001209,

    /**
     * [610001218] Die übergebene Kombination von Bearbeitungsflags ist nicht erlaubt.
     */
    ERIC_GLOBAL_UNGUELTIGE_FLAG_KOMBINATION = 610001218,

    /**
     * [610001220] Der Erste-Seite-Druck ist nur für Steuerberater bei nicht-authentifizierten Einkommenssteuerfällen ab Veranlagungszeitraum 2010 ohne Versandanforderung erlaubt.
     */
    ERIC_GLOBAL_ERSTE_SEITE_DRUCK_NICHT_UNTERSTUETZT = 610001220,

    /**
     * [610001222] Die angegebenen Parameter sind ungültig oder unvollständig.
     */
    ERIC_GLOBAL_UNGUELTIGER_PARAMETER = 610001222,

    /**
     * [610001224] Für das angegebene Verfahren wird der Druck nicht unterstützt.
     */
    ERIC_GLOBAL_DRUCK_FUER_VERFAHREN_NICHT_ERLAUBT = 610001224,

    /**
     * [610001225] Die Versandart ist für die angegebene Datenartversion nicht erlaubt.
     */
    ERIC_GLOBAL_VERSAND_ART_NICHT_UNTERSTUETZT = 610001225,

    /**
     * [610001226] Die Version eines der angegebenen Parameter ist ungültig.
     */
    ERIC_GLOBAL_UNGUELTIGE_PARAMETER_VERSION = 610001226,

    /**
     * [610001227] Für das Verfahren Datenabholung wurde ein illegales Transferhandle angegeben.
     */
    ERIC_GLOBAL_TRANSFERHANDLE = 610001227,

    /**
     * [610001228] Die Initialisierung eines Plugins ist fehlgeschlagen.
     */
    ERIC_GLOBAL_PLUGININITIALISIERUNG = 610001228,

    /**
     * [610001229] Die Versionen der im Logfile genannten ERiC-Dateien sind nicht kompatibel. (Siehe eric.log.)
     */
    ERIC_GLOBAL_INKOMPATIBLE_VERSIONEN = 610001229,

    /**
     * [610001230] Das im XML-Element "<Verschluesselung>" angegebene Verschlüsselungsverfahren wird vom ERiC nicht unterstützt.
     */
    ERIC_GLOBAL_VERSCHLUESSELUNGSVERFAHREN_NICHT_UNTERSTUETZT = 610001230,

    /**
     * [610001231] Der Aufruf eine API-Funktion des ERiCs darf erst dann erfolgen, wenn ein vorheriger Aufruf zurückgekehrt ist.
     */
    ERIC_GLOBAL_MEHRFACHAUFRUFE_NICHT_UNTERSTUETZT = 610001231,

    /**
     * [610001404] Das Bundesland/Finanzamt mit der angegebenen Nummer nimmt bei der angegebenen Steuerart am ELSTER-Verfahren nicht teil.
     */
    ERIC_GLOBAL_UTI_COUNTRY_NOT_SUPPORTED = 610001404,

    /**
     * [610001501] Ungültige IBAN: IBAN muss aus zweistelligem Ländercode gefolgt von zweistelliger Prüfziffer gefolgt von der Basic Bank Account Number bestehen.
     */
    ERIC_GLOBAL_IBAN_FORMALER_FEHLER = 610001501,

    /**
     * [610001502] Ungültige IBAN: Der angegebene Ländercode ist ungültig oder wird aktuell im ELSTER-Verfahren nicht unterstützt.
     */
    ERIC_GLOBAL_IBAN_LAENDERCODE_FEHLER = 610001502,

    /**
     * [610001503] Ungültige IBAN: Die angegebene IBAN entspricht nicht dem für das angegebene Land definierten formalen Aufbau der IBAN oder die IBAN ist unzulässig.
     */
    ERIC_GLOBAL_IBAN_LANDESFORMAT_FEHLER = 610001503,

    /**
     * [610001504] Ungültige IBAN: Die Prüfziffernberechnung zur angegebenen IBAN führt zu einer abweichenden Prüfziffer.
     */
    ERIC_GLOBAL_IBAN_PRUEFZIFFER_FEHLER = 610001504,

    /**
     * [610001510] Ungültiger BIC: Der formale Aufbau des angegeben BIC ist ungültig.
     */
    ERIC_GLOBAL_BIC_FORMALER_FEHLER = 610001510,

    /**
     * [610001511] Ungültiger BIC: Der angegebene Ländercode ist ungültig oder wird aktuell im ELSTER-Verfahren nicht unterstützt.
     */
    ERIC_GLOBAL_BIC_LAENDERCODE_FEHLER = 610001511,

    /**
     * [610001519] Die angegebene Zulassungsnummer entspricht nicht den Längenvorgaben. Es sind maximal 6 Stellen erlaubt.
     */
    ERIC_GLOBAL_ZULASSUNGSNUMMER_ZU_LANG = 610001519,

    /**
     * [610001525] Die übergebene IDNummer ist ungültig.
     */
    ERIC_GLOBAL_IDNUMMER_UNGUELTIG = 610001525,

    /**
     * [610001526] Es wurde der Parameter NULL übergeben.
     */
    ERIC_GLOBAL_NULL_PARAMETER = 610001526,

    /**
     * [610001527] Das übergebene Einheitswert-Aktenzeichen ist ungültig.
     */
    ERIC_GLOBAL_EWAZ_UNGUELTIG = 610001527,

    /**
     * [610001528] Das übergebene Landeskürzel ist unbekannt oder leer.
     */
    ERIC_GLOBAL_EWAZ_LANDESKUERZEL_UNBEKANNT = 610001528,

    /**
     * [610001851] Update des ERiC erforderlich. Starten Sie nun das Update.
     */
    ERIC_GLOBAL_UPDATE_NECESSARY = 610001851,

    /**
     * [610001860] Ungültiger Name für Einstellung.
     */
    ERIC_GLOBAL_EINSTELLUNG_NAME_UNGUELTIG = 610001860,

    /**
     * [610001861] Ungültiger Wert für Einstellung.
     */
    ERIC_GLOBAL_EINSTELLUNG_WERT_UNGUELTIG = 610001861,

    /**
     * [610001862] Fehler beim Dekodieren.
     */
    ERIC_GLOBAL_ERR_DEKODIEREN = 610001862,

    /**
     * [610001863] Die aufgerufene Funktion wird nicht unterstützt.
     */
    ERIC_GLOBAL_FUNKTION_NICHT_UNTERSTUETZT = 610001863,

    /**
     * [610001865] Fehler im übergebenen EDS-XML: In den Sammeldaten wurde ein Nutzdatenticket für mehrere Steuerfälle verwendet. Für jeden Steuerfall muss jedoch ein eigenes Nutzdatenticket angegeben werden.
     */
    ERIC_GLOBAL_NUTZDATENTICKETS_NICHT_EINDEUTIG = 610001865,

    /**
     * [610001866] Fehler im übergebenen EDS-XML: Bei den Sammeldaten wurden unterschiedliche Versionen des Nutzdaten-Headers verwendet. Innerhalb einer Datenlieferung ist jedoch nur eine Nutzdaten-Header-Version zulässig.
     */
    ERIC_GLOBAL_NUTZDATENHEADERVERSIONEN_UNEINHEITLICH = 610001866,

    /**
     * [610001867] Fehler im übergebenen EDS-XML: Es wurden Fälle für mehrere Bundesländer angegeben. Innerhalb einer Datenlieferung dürfen jedoch nur Fälle für ein Bundesland angegeben werden.
     */
    ERIC_GLOBAL_BUNDESLAENDER_UNEINHEITLICH = 610001867,

    /**
     * [610001868] Fehler im übergebenen EDS-XML: Es wurden Fälle für unterschiedliche Jahre angegeben. Innerhalb einer Datenlieferung dürfen jedoch nur Fälle für ein und dasselbe Jahr angegeben werden.
     */
    ERIC_GLOBAL_ZEITRAEUME_UNEINHEITLICH = 610001868,

    /**
     * [610001869] Fehler im übergebenen EDS-XML: Der Inhalt des Nutzdaten-Elements "<Empfaenger>" ist für diese Datenart nicht korrekt.
     */
    ERIC_GLOBAL_NUTZDATENHEADER_EMPFAENGER_NICHT_KORREKT = 610001869,

    /**
     * [610101200] Allgemeiner Kommunikationsfehler.
     */
    ERIC_TRANSFER_COM_ERROR = 610101200,

    /**
     * [610101201] Dieser Vorgang wird von der aufgerufenen Funktion nicht unterstützt.
     */
    ERIC_TRANSFER_VORGANG_NICHT_UNTERSTUETZT = 610101201,

    /**
     * [610101210] Fehler im Transferheader. Der ELSTER-Annahmeserver hat einen Fehler zurückgemeldet. Bitte werten Sie die Serverantwort aus.
     */
    ERIC_TRANSFER_ERR_XML_THEADER = 610101210,

    /**
     * [610101251] Es wurden ungültige Parameter übergeben.
     */
    ERIC_TRANSFER_ERR_PARAM = 610101251,

    /**
     * [610101253] Im XML-String konnte der Text "</DatenTeil>" nicht gefunden werden.
     */
    ERIC_TRANSFER_ERR_DATENTEILENDNOTFOUND = 610101253,

    /**
     * [610101255] Im XML-String konnte der Text "<DatenLieferant>" nicht gefunden werden.
     */
    ERIC_TRANSFER_ERR_BEGINDATENLIEFERANT = 610101255,

    /**
     * [610101256] Im XML-String konnte der Text "</DatenLieferant>" nicht gefunden werden.
     */
    ERIC_TRANSFER_ERR_ENDDATENLIEFERANT = 610101256,

    /**
     * [610101257] Im XML-String konnte der Text "<TransportSchluessel>" nicht gefunden werden.
     */
    ERIC_TRANSFER_ERR_BEGINTRANSPORTSCHLUESSEL = 610101257,

    /**
     * [610101258] Im XML-String konnte der Text "</TransportSchluessel>" nicht gefunden werden.
     */
    ERIC_TRANSFER_ERR_ENDTRANSPORTSCHLUESSEL = 610101258,

    /**
     * [610101259] Im XML-String konnte der Text "<DatenGroesse>" nicht gefunden werden.
     */
    ERIC_TRANSFER_ERR_BEGINDATENGROESSE = 610101259,

    /**
     * [610101260] Im XML-String konnte der Text "</DatenGroesse>" nicht gefunden werden.
     */
    ERIC_TRANSFER_ERR_ENDDATENGROESSE = 610101260,

    /**
     * [610101271] Beim Datenaustausch ist ein Fehler aufgetreten.
     */
    ERIC_TRANSFER_ERR_SEND = 610101271,

    /**
     * [610101274] Die Antwortdaten waren nicht PKCS#7-verschlüsselt.
     */
    ERIC_TRANSFER_ERR_NOTENCRYPTED = 610101274,

    /**
     * [610101276] Verbindung zum ProxyServer konnte nicht aufgebaut werden.
     */
    ERIC_TRANSFER_ERR_PROXYCONNECT = 610101276,

    /**
     * [610101278] Zu den Servern konnte keine Verbindung aufgebaut werden.
     */
    ERIC_TRANSFER_ERR_CONNECTSERVER = 610101278,

    /**
     * [610101279] Von der Clearingstelle konnte keine Antwort empfangen werden.
     */
    ERIC_TRANSFER_ERR_NORESPONSE = 610101279,

    /**
     * [610101280] Der Proxyserver erwartet Anmeldedaten.
     */
    ERIC_TRANSFER_ERR_PROXYAUTH = 610101280,

    /**
     * [610101282] Fehler bei der Initialisierung des Versands, Details stehen ggf. im Logfile (eric.log).
     */
    ERIC_TRANSFER_ERR_SEND_INIT = 610101282,

    /**
     * [610101283] Bei der Kommunikation mit dem Server kam es zu einer Zeitüberschreitung.
     */
    ERIC_TRANSFER_ERR_TIMEOUT = 610101283,

    /**
     * [610101284] Es wurde kein gültiger Port für den Proxy angegeben.
     */
    ERIC_TRANSFER_ERR_PROXYPORT_INVALID = 610101284,

    /**
     * [610101291] Sonstiger, nicht definierter Fehler aufgetreten.
     */
    ERIC_TRANSFER_ERR_OTHER = 610101291,

    /**
     * [610101292] Fehler im NutzdatenHeader. Der ELSTER-Annahmeserver hat einen Fehler zurückgemeldet. Bitte werten Sie die Serverantwort aus. Bei Sammeldaten sind alle Nutzdatenblöcke zu prüfen, um den fehlerhaften Datensatz identifizieren zu können.
     */
    ERIC_TRANSFER_ERR_XML_NHEADER = 610101292,

    /**
     * [610101293] Das XML liegt im falschen Encoding vor.
     */
    ERIC_TRANSFER_ERR_XML_ENCODING = 610101293,

    /**
     * [610101294] Im XML-String konnte der Text "</SigUser>" nicht gefunden werden.
     */
    ERIC_TRANSFER_ERR_ENDSIGUSER = 610101294,

    /**
     * [610101295] Im XML-String konnte ein Tag nicht gefunden werden.
     */
    ERIC_TRANSFER_ERR_XMLTAG_NICHT_GEFUNDEN = 610101295,

    /**
     * [610101297] Das XML-Element "<DatenTeil>" konnte nicht gelesen werden.
     */
    ERIC_TRANSFER_ERR_DATENTEILFEHLER = 610101297,

    /**
     * [610101500] Es konnte kein Ad Hoc-Zertifikat fuer den Personalausweis oder den Aufenthaltstitel erzeugt bzw. gefunden werden, Details stehen ggf. im Logfile (eric.log).
     */
    ERIC_TRANSFER_EID_ZERTIFIKATFEHLER = 610101500,

    /**
     * [610101510] Für die Identifikationsnummer des Benutzers existiert kein Konto bei ELSTER.
     */
    ERIC_TRANSFER_EID_KEINKONTO = 610101510,

    /**
     * [610101511] Dem Benutzer konnte keine eindeutige Identifikationsnummer zugeordnet werden.
     */
    ERIC_TRANSFER_EID_IDNRNICHTEINDEUTIG = 610101511,

    /**
     * [610101512] Das nPA-Servlet konnte keine Verbindung zum eID-Server aufbauen.
     */
    ERIC_TRANSFER_EID_SERVERFEHLER = 610101512,

    /**
     * [610101520] Der eID-Client ist nicht erreichbar. Wahrscheinlich wurde er nicht gestartet oder die übergebene lokale URL ist nicht korrekt
     */
    ERIC_TRANSFER_EID_KEINCLIENT = 610101520,

    /**
     * [610101521] Der eID-Client hat einen Fehler gemeldet. Details zu dem Fehler finden Sie im Log des eID-Clients oder ggf. im ERiC Logfile (eric.log).
     */
    ERIC_TRANSFER_EID_CLIENTFEHLER = 610101521,

    /**
     * [610101522] Es konnten nicht alle benötigten Datenfelder des Personalausweises ausgelesen werden. Bitte prüfen Sie über die Funktion "Selbstauskunft" des eID-Clients, ob folgende Daten von Ihrem Personalausweis korrekt bereitgestellt werden: Familienname, Vorname(n), Geburtsdatum, Anschrift (mit Postleitzahl) und Dokumentenart.
     */
    ERIC_TRANSFER_EID_FEHLENDEFELDER = 610101522,

    /**
     * [610101523] Das Auslesen der Daten aus dem Personalausweis wurde vom Anwender abgebrochen.
     */
    ERIC_TRANSFER_EID_IDENTIFIKATIONABGEBROCHEN = 610101523,

    /**
     * [610101524] Der Personalausweis wird von einem anderen Vorgang blockiert. Beenden Sie den anderen Vorgang und versuchen Sie es dann erneut.
     */
    ERIC_TRANSFER_EID_NPABLOCKIERT = 610101524,

    /**
     * [610201016] Fehler bei der Schlüsselerzeugung.
     */
    ERIC_CRYPT_ERROR_CREATE_KEY = 610201016,

    /**
     * [610201101] eSigner: Ungültiges Token Handle.
     */
    ERIC_CRYPT_E_INVALID_HANDLE = 610201101,

    /**
     * [610201102] eSigner: Zu viele Sessions geöffnet.
     */
    ERIC_CRYPT_E_MAX_SESSION = 610201102,

    /**
     * [610201103] eSigner: Überlastung.
     */
    ERIC_CRYPT_E_BUSY = 610201103,

    /**
     * [610201104] eSigner: Speicherzuordnungsfehler.
     */
    ERIC_CRYPT_E_OUT_OF_MEM = 610201104,

    /**
     * [610201105] eSigner: Ungültiger PSE Pfad.
     */
    ERIC_CRYPT_E_PSE_PATH = 610201105,

    /**
     * [610201106] eSigner: Es wurde ein falsches Passwort bzw. eine falsche PIN angegeben.
     */
    ERIC_CRYPT_E_PIN_WRONG = 610201106,

    /**
     * [610201107] eSigner: Das Passwort bzw. die PIN ist gesperrt.
     */
    ERIC_CRYPT_E_PIN_LOCKED = 610201107,

    /**
     * [610201108] eSigner: Fehler beim Lesen des PKCS#7-Objekts.
     */
    ERIC_CRYPT_E_P7_READ = 610201108,

    /**
     * [610201109] eSigner: Fehler beim PKCS#7 Dekodieren.
     */
    ERIC_CRYPT_E_P7_DECODE = 610201109,

    /**
     * [610201110] eSigner: Entschlüsselungszertifikat nicht in Empfängerliste enthalten.
     */
    ERIC_CRYPT_E_P7_RECIPIENT = 610201110,

    /**
     * [610201111] eSigner: Fehler beim Lesen des PKCS#12-Objekts.
     */
    ERIC_CRYPT_E_P12_READ = 610201111,

    /**
     * [610201112] eSigner: Fehler beim Dekodieren des PKCS#12-Objekts.
     */
    ERIC_CRYPT_E_P12_DECODE = 610201112,

    /**
     * [610201113] eSigner: Fehler beim Zugriff auf Soft-PSE-Signaturschlüssel.
     */
    ERIC_CRYPT_E_P12_SIG_KEY = 610201113,

    /**
     * [610201114] eSigner: Fehler beim Zugriff auf Soft-PSE Entschlüsselungsschlüssel.
     */
    ERIC_CRYPT_E_P12_ENC_KEY = 610201114,

    /**
     * [610201115] eSigner: Fehler beim Zugriff auf Hard-Token Signaturschlüssel.
     */
    ERIC_CRYPT_E_P11_SIG_KEY = 610201115,

    /**
     * [610201116] eSigner: Fehler beim Zugriff auf Hard-Token Entschlüsselungsschlüssel.
     */
    ERIC_CRYPT_E_P11_ENC_KEY = 610201116,

    /**
     * [610201117] eSigner: Fehler beim Parsen der XML-Eingabedatei.
     */
    ERIC_CRYPT_E_XML_PARSE = 610201117,

    /**
     * [610201118] eSigner: Fehler beim Erzeugen des XML-Signaturasts.
     */
    ERIC_CRYPT_E_XML_SIG_ADD = 610201118,

    /**
     * [610201119] eSigner: XML-Signaturtag nicht vorhanden.
     */
    ERIC_CRYPT_E_XML_SIG_TAG = 610201119,

    /**
     * [610201120] eSigner: Fehler bei XML-Signaturerzeugung.
     */
    ERIC_CRYPT_E_XML_SIG_SIGN = 610201120,

    /**
     * [610201121] eSigner: Parameter-Fehler, unbekanntes Encoding.
     */
    ERIC_CRYPT_E_ENCODE_UNKNOWN = 610201121,

    /**
     * [610201122] eSigner: Encoding-Fehler.
     */
    ERIC_CRYPT_E_ENCODE_ERROR = 610201122,

    /**
     * [610201123] eSigner: XML Initialisierungsfehler.
     */
    ERIC_CRYPT_E_XML_INIT = 610201123,

    /**
     * [610201124] eSigner: Fehler beim Verschlüsseln.
     */
    ERIC_CRYPT_E_ENCRYPT = 610201124,

    /**
     * [610201125] eSigner: Fehler beim Entschlüsseln.
     */
    ERIC_CRYPT_E_DECRYPT = 610201125,

    /**
     * [610201126] eSigner: Keine Signaturkarte eingesteckt (PKCS#11).
     */
    ERIC_CRYPT_E_P11_SLOT_EMPTY = 610201126,

    /**
     * [610201127] eSigner: Keine Signatur-/Verschlüsselungs-Zertifikate/-Schlüssel gefunden (PKCS#11).
     */
    ERIC_CRYPT_E_NO_SIG_ENC_KEY = 610201127,

    /**
     * [610201128] eSigner: PKCS11 bzw. PC/SC Library fehlt oder ist nicht ausführbar.
     */
    ERIC_CRYPT_E_LOAD_DLL = 610201128,

    /**
     * [610201129] eSigner: Der PC/SC Dienst ist nicht gestartet.
     */
    ERIC_CRYPT_E_NO_SERVICE = 610201129,

    /**
     * [610201130] eSigner: Unbekannte Ausnahme aufgetreten.
     */
    ERIC_CRYPT_E_ESICL_EXCEPTION = 610201130,

    /**
     * [610201144] eSigner: CA Tokentyp und interner Tokentyp stimmen nicht überein.
     */
    ERIC_CRYPT_E_TOKEN_TYPE_MISMATCH = 610201144,

    /**
     * [610201146] eSigner: Temporäres PKCS#12-Token kann nicht erzeugt werden.
     */
    ERIC_CRYPT_E_P12_CREATE = 610201146,

    /**
     * [610201147] eSigner: Zertifikatskette konnte nicht verifiziert werden.
     */
    ERIC_CRYPT_E_VERIFY_CERT_CHAIN = 610201147,

    /**
     * [610201148] eSigner: PKCS#11 Engine mit anderer Bibliothek belegt.
     */
    ERIC_CRYPT_E_P11_ENGINE_LOADED = 610201148,

    /**
     * [610201149] eSigner: Aktion vom Benutzer abgebrochen.
     */
    ERIC_CRYPT_E_USER_CANCEL = 610201149,

    /**
     * [610201200] Fehler beim Zugriff auf Zertifikat.
     */
    ERIC_CRYPT_ZERTIFIKAT = 610201200,

    /**
     * [610201201] Fehler bei Signaturerzeugung.
     */
    ERIC_CRYPT_SIGNATUR = 610201201,

    /**
     * [610201203] Das Format der PSE wird nicht unterstützt.
     */
    ERIC_CRYPT_NICHT_UNTERSTUETZTES_PSE_FORMAT = 610201203,

    /**
     * [610201205] Für die ausgewählte Operation muss ein Passwort bzw. eine PIN angegeben werden.
     */
    ERIC_CRYPT_PIN_BENOETIGT = 610201205,

    /**
     * [610201206] Das gewünschte Passwort ist nicht sicher genug (z.B. zu kurz).
     */
    ERIC_CRYPT_PIN_STAERKE_NICHT_AUSREICHEND = 610201206,

    /**
     * [610201208] Interner Fehler aufgetreten. Details stehen ggf. im Logfile (eric.log).
     */
    ERIC_CRYPT_E_INTERN = 610201208,

    /**
     * [610201209] Der angegebene Zertifikatspfad ist kein Verzeichnis.
     */
    ERIC_CRYPT_ZERTIFIKATSPFAD_KEIN_VERZEICHNIS = 610201209,

    /**
     * [610201210] Im angegebenen Verzeichnis existiert bereits ein Bestandteil eines ERiC-Zertifikats.
     */
    ERIC_CRYPT_ZERTIFIKATSDATEI_EXISTIERT_BEREITS = 610201210,

    /**
     * [610201211] Das gewünschte Passwort enthält ungültige Zeichen (z.B. Umlaute).
     */
    ERIC_CRYPT_PIN_ENTHAELT_UNGUELTIGE_ZEICHEN = 610201211,

    /**
     * [610201212] eSigner: Der Abrufcode besitzt eine falsche Struktur oder enthält ungültige Zeichen.
     */
    ERIC_CRYPT_E_INVALID_PARAM_ABC = 610201212,

    /**
     * [610201213] Das übergebene Zertifikat weist Inkonsistenzen auf und kann deswegen nicht verwendet werden. Bitte verwenden Sie ein anderes oder erzeugen und verwenden Sie ein neues Zertifikat.
     */
    ERIC_CRYPT_CORRUPTED = 610201213,

    /**
     * [610201214] Die aufgerufene Funktion unterstützt den neuen Personalausweis (nPA) und den elektronischen Aufenthaltstitel (eAT) nicht.
     */
    ERIC_CRYPT_EIDKARTE_NICHT_UNTERSTUETZT = 610201214,

    /**
     * [610201215] Es ist keine Karte/kein Stick eingesteckt
     */
    ERIC_CRYPT_E_SC_SLOT_EMPTY = 610201215,

    /**
     * [610201216] Kein unterstütztes Applet gefunden
     */
    ERIC_CRYPT_E_SC_NO_APPLET = 610201216,

    /**
     * [610201217] Fehler in der Kartensession
     */
    ERIC_CRYPT_E_SC_SESSION = 610201217,

    /**
     * [610201218] P11 Signaturzertifikat fehlt
     */
    ERIC_CRYPT_E_P11_NO_SIG_CERT = 610201218,

    /**
     * [610201219] P11 Der initiale Tokenzugriff ist fehlgeschlagen
     */
    ERIC_CRYPT_E_P11_INIT_FAILED = 610201219,

    /**
     * [610201220] P11 Verschlüsselungszertifikat fehlt
     */
    ERIC_CRYPT_E_P11_NO_ENC_CERT = 610201220,

    /**
     * [610201221] P12 Signaturzertifikat fehlt
     */
    ERIC_CRYPT_E_P12_NO_SIG_CERT = 610201221,

    /**
     * [610201222] P12 Verschlüsselungszertifikat fehlt
     */
    ERIC_CRYPT_E_P12_NO_ENC_CERT = 610201222,

    /**
     * [610201223] PC/SC Der Zugriff auf den Entschlüsselungsschlüssel ist fehlgeschlagen
     */
    ERIC_CRYPT_E_SC_ENC_KEY = 610201223,

    /**
     * [610201224] PC/SC Signaturzertifikat fehlt
     */
    ERIC_CRYPT_E_SC_NO_SIG_CERT = 610201224,

    /**
     * [610201225] PC/SC Verschlüsselungszertifikat fehlt
     */
    ERIC_CRYPT_E_SC_NO_ENC_CERT = 610201225,

    /**
     * [610201226] PC/SC Der initiale Tokenzugriff ist fehlgeschlagen
     */
    ERIC_CRYPT_E_SC_INIT_FAILED = 610201226,

    /**
     * [610201227] PC/SC Der Zugriff auf den Signaturschlüssel ist fehlgeschlagen
     */
    ERIC_CRYPT_E_SC_SIG_KEY = 610201227,

    /**
     * [610301001] Verarbeitung fehlerhaft, keine genaueren Informationen vorhanden.
     */
    ERIC_IO_FEHLER = 610301001,

    /**
     * [610301005] Der Dateiaufbau ist nicht korrekt.
     */
    ERIC_IO_DATEI_INKORREKT = 610301005,

    /**
     * [610301006] Fehler beim Parsen der Eingabedaten. Details stehen im Logfile (eric.log).
     */
    ERIC_IO_PARSE_FEHLER = 610301006,

    /**
     * [610301007] Die Generierung des Nutzdatensatzes ist fehlgeschlagen.
     */
    ERIC_IO_NDS_GENERIERUNG_FEHLGESCHLAGEN = 610301007,

    /**
     * [610301010] Interner Fehler, der Masterdatenservice ist nicht verfügbar.
     */
    ERIC_IO_MASTERDATENSERVICE_NICHT_VERFUEGBAR = 610301010,

    /**
     * [610301014] Es wurden ungültige Steuerzeichen im Nutzdatensatz gefunden.
     */
    ERIC_IO_STEUERZEICHEN_IM_NDS = 610301014,

    /**
     * [610301031] Die Versionsinformationen der ERiC-Bibliotheken konnten nicht ausgelesen werden.
     */
    ERIC_IO_VERSIONSINFORMATIONEN_NICHT_GEFUNDEN = 610301031,

    /**
     * [610301104] Der Wert im Transferheader-Element "Verfahren" wird vom verwendeten Reader nicht unterstützt.
     */
    ERIC_IO_FALSCHES_VERFAHREN = 610301104,

    /**
     * [610301105] Es wurde mehr als ein Steuerfall in der Eingabedatei gefunden.
     */
    ERIC_IO_READER_MEHRFACHE_STEUERFAELLE = 610301105,

    /**
     * [610301106] Es wurden unerwartete Elemente in der Eingabedatei gefunden, Details stehen ggf. im Logfile (eric.log).
     */
    ERIC_IO_READER_UNERWARTETE_ELEMENTE = 610301106,

    /**
     * [610301107] Es wurden formale Fehler in der Eingabedatei gefunden, Details stehen ggf. im Logfile (eric.log).
     */
    ERIC_IO_READER_FORMALE_FEHLER = 610301107,

    /**
     * [610301108] Die Eingabedaten lagen nicht im Encoding UTF-8 vor, oder es war kein Encoding spezifiziert.
     */
    ERIC_IO_READER_FALSCHES_ENCODING = 610301108,

    /**
     * [610301109] Es wurde mehr als ein "Nutzdaten"-Element in der Eingabedatei gefunden.
     */
    ERIC_IO_READER_MEHRFACHE_NUTZDATEN_ELEMENTE = 610301109,

    /**
     * [610301110] Es wurde mehr als ein Nutzdatenblock in der Eingabedatei gefunden.
     */
    ERIC_IO_READER_MEHRFACHE_NUTZDATENBLOCK_ELEMENTE = 610301110,

    /**
     * [610301111] Der im Transferheader-Element "Datenart" angegebene Wert ist unbekannt.
     */
    ERIC_IO_UNBEKANNTE_DATENART = 610301111,

    /**
     * [610301114] Ungültiger oder fehlender Wert für den Untersachbereich.
     */
    ERIC_IO_READER_UNTERSACHBEREICH_UNGUELTIG = 610301114,

    /**
     * [610301115] Es wurden zu viele Nutzdatenblöcke in der Eingabedatei gefunden.
     */
    ERIC_IO_READER_ZU_VIELE_NUTZDATENBLOCK_ELEMENTE = 610301115,

    /**
     * [610301150] Es wurden ungültige Steuerzeichen im TransferHeader-Element gefunden.
     */
    ERIC_IO_READER_STEUERZEICHEN_IM_TRANSFERHEADER = 610301150,

    /**
     * [610301151] Es wurden ungültige Steuerzeichen im NutzdatenHeader-Element gefunden.
     */
    ERIC_IO_READER_STEUERZEICHEN_IM_NUTZDATENHEADER = 610301151,

    /**
     * [610301152] Es wurden ungültige Steuerzeichen im Nutzdaten-Element gefunden.
     */
    ERIC_IO_READER_STEUERZEICHEN_IN_DEN_NUTZDATEN = 610301152,

    /**
     * [610301190] Ein Nutzdatenblock enthält zu viele Anhänge. Details stehen im Logfile (eric.log).
     */
    ERIC_IO_READER_ZU_VIELE_ANHAENGE = 610301190,

    /**
     * [610301191] Ein Anhang ist zu groß. Details stehen im Logfile (eric.log).
     */
    ERIC_IO_READER_ANHANG_ZU_GROSS = 610301191,

    /**
     * [610301192] Die Gesamtgröße aller Anhange in einem Nutzdatenblock ist zu groß. Details stehen im Logfile (eric.log).
     */
    ERIC_IO_READER_ANHAENGE_ZU_GROSS = 610301192,

    /**
     * [610301200] Es traten Fehler beim Validieren des XML auf. Details stehen im Logfile (eric.log).
     */
    ERIC_IO_READER_SCHEMA_VALIDIERUNGSFEHLER = 610301200,

    /**
     * [610301201] Eine XML-Entity konnte nicht aufgelöst werden.
     */
    ERIC_IO_READER_UNBEKANNTE_XML_ENTITY = 610301201,

    /**
     * [610301252] Im XML-String konnte der Text "<DatenTeil>" nicht gefunden werden.
     */
    ERIC_IO_DATENTEILNOTFOUND = 610301252,

    /**
     * [610301253] Im XML-String konnte der Text "</DatenTeil>" nicht gefunden werden.
     */
    ERIC_IO_DATENTEILENDNOTFOUND = 610301253,

    /**
     * [610301300] Falsche Übergabeparameter für die Funktion.
     */
    ERIC_IO_UEBERGABEPARAMETER_FEHLERHAFT = 610301300,

    /**
     * [610301400] Der Parameter enthält ungültige UTF-8 Multibytesequenzen.
     */
    ERIC_IO_UNGUELTIGE_UTF8_SEQUENZ = 610301400,

    /**
     * [610301401] Der Parameter enthält mindestens ein unzulässiges Zeichen.
     */
    ERIC_IO_UNGUELTIGE_ZEICHEN_IN_PARAMETER = 610301401,

    /**
     * [610501001] Verarbeitung fehlerhaft, keine genaueren Informationen vorhanden.
     */
    ERIC_PRINT_INTERNER_FEHLER = 610501001,

    /**
     * [610501002] Keine Druckvorlage für die angegebene Kombination aus Unterfallart und Veranlagungszeitraum gefunden. Bitte prüfen Sie die installierten Druckvorlagen.
     */
    ERIC_PRINT_DRUCKVORLAGE_NICHT_GEFUNDEN = 610501002,

    /**
     * [610501004] Es wurde ein falscher Dateipfad angegeben, es fehlen Zugriffsrechte oder die Datei wird aktuell von einer anderen Anwendung verwendet.
     */
    ERIC_PRINT_UNGUELTIGER_DATEI_PFAD = 610501004,

    /**
     * [610501007] ERiCPrint wurde nicht richtig initialisiert. Eventuell wurde ERiC nicht richtig initialisiert?
     */
    ERIC_PRINT_INITIALISIERUNG_FEHLERHAFT = 610501007,

    /**
     * [610501008] Das zu verwendende Format bzw. der Zielklient sind nicht bekannt.
     */
    ERIC_PRINT_AUSGABEZIEL_UNBEKANNT = 610501008,

    /**
     * [610501009] Der Beginn des Ausdruckprozesses schlug fehl. Eventuell konnten notwendige Ressourcen nicht allokiert werden.
     */
    ERIC_PRINT_ABBRUCH_DRUCKVORBEREITUNG = 610501009,

    /**
     * [610501010] Während der Ausgabe der Inhalte ist ein Fehler aufgetreten.
     */
    ERIC_PRINT_ABBRUCH_GENERIERUNG = 610501010,

    /**
     * [610501011] Die Kombination aus Unterfallart und Veranlagungszeitraum wird nicht unterstützt.
     */
    ERIC_PRINT_STEUERFALL_NICHT_UNTERSTUETZT = 610501011,

    /**
     * [610501012] Der übergebene Fußtext ist zu lang.
     */
    ERIC_PRINT_FUSSTEXT_ZU_LANG = 610501012,
}
