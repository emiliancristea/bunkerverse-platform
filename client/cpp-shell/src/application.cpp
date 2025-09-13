#include "application.h"
#include <QQmlApplicationEngine>

Application::Application(QObject* parent)
    : QObject(parent), m_engine(nullptr) {
}

Application::~Application() {
    delete m_engine;
}