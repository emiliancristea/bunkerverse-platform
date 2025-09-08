#include "application.h"
#include <QQmlApplicationEngine>

Application::Application(QObject *parent)
    : QObject(parent)
    , m_engine(nullptr)
{
}

Application::~Application()
{
    delete m_engine;
}

void Application::initialize()
{
    m_engine = new QQmlApplicationEngine(this);
    
    // Future CXX-Qt integration will be added here
    // This will bridge Rust application logic with QML UI
}